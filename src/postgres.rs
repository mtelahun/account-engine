use std::{iter::zip, ops::Deref, str::FromStr, sync::Arc, time::Duration};

use async_trait::async_trait;
use chronoutil::RelativeDuration;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use postgres_types::ToSql;
use tokio_postgres::{Config, NoTls, Row};

use crate::{
    domain::{
        array_long_string::ArrayLongString,
        array_short_string::ArrayShortString,
        ids::{InterimPeriodId, JournalId},
        ledger_xact_type_code, AccountId, GeneralLedgerId, JournalTransactionId,
        LedgerXactTypeCode, PeriodId, XactType,
    },
    entity::{
        accounting_period, general_ledger, interim_accounting_period, journal, journal_entry,
        journal_transaction, journal_transaction_line, journal_transaction_line_account,
        journal_transaction_line_ledger, journal_transaction_record, ledger, ledger_intermediate,
        ledger_leaf, ledger_line, ledger_transaction, ledger_xact_type,
        ledgers::account::ledger_derived, InterimType, LedgerKey, LedgerType, PostingRef,
        TransactionState,
    },
    orm::{AccountRepository, OrmError, RepositoryEntity},
};

const MAX_OPEN_CONNECTIONS: u64 = 32;
const MAX_IDLE_CONNECTIONS: u64 = 8;
const MAX_TIMEOUT_SECONDS: u64 = 15;

type PgConn = Connection<PgConnectionManager<NoTls>>;
type PgPool = Pool<PgConnectionManager<NoTls>>;

fn vec_id2uuid<T: Deref<Target = uuid::Uuid>>(ids: Vec<T>) -> Vec<uuid::Uuid> {
    let mut uuids = Vec::<uuid::Uuid>::new();
    for i in ids {
        uuids.push(*i)
    }

    uuids
}

pub struct PgStore {
    inner: Arc<Inner>,
}

pub struct Inner {
    pool: PgPool,
}

#[derive(Clone, Copy, Debug)]
pub struct SqlData<'a> {
    statement: &'a str,
    params: &'a [&'a (dyn ToSql + Sync)],
}

impl From<Row> for general_ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayLongString::from_str(value.get("name")).unwrap_or_default(),
            root: value.get("root"),
            currency_code: value.get("currency_code"),
        }
    }
}

impl From<Row> for ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: ArrayLongString::from_str(value.get("name")).unwrap_or_default(),
            ledger_no: ArrayShortString::from_str(value.get("ledger_no")).unwrap_or_default(),
            ledger_type: value.get("ledger_type"),
            parent_id: value.get("parent_id"),
            currency_code: value.get("currency_code"),
        }
    }
}

impl From<Row> for ledger_intermediate::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
        }
    }
}

impl From<Row> for ledger_leaf::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
        }
    }
}

impl From<Row> for ledger_derived::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            account_book_id: value.get("account_book_id"),
        }
    }
}

impl From<Row> for journal::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            name: value.get("name"),
            code: value.get("code"),
        }
    }
}

impl From<Row> for journal_transaction_record::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            explanation: value.get("explanation"),
        }
    }
}

impl From<Row> for journal_transaction_line_ledger::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            ledger_id: value.get("ledger_id"),
            xact_type: value.get("xact_type"),
            amount: value.get("amount"),
            state: value.get("state"),
            posting_ref: value.get("posting_ref"),
        }
    }
}

impl From<Row> for journal_transaction_line_account::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            journal_id: value.get("journal_id"),
            timestamp: value.get("timestamp"),
            account_id: value.get("account_id"),
            xact_type: value.get("xact_type"),
            amount: value.get("amount"),
            state: value.get("state"),
            posting_ref: value.get("posting_ref"),
        }
    }
}

impl From<Row> for accounting_period::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            fiscal_year: value.get("fiscal_year"),
            period_start: value.get("period_start"),
            period_end: value.get("period_end"),
            period_type: value.get("period_type"),
        }
    }
}

impl From<Row> for interim_accounting_period::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            id: value.get("id"),
            parent_id: value.get("parent_id"),
            start: value.get("interim_start"),
            end: value.get("interim_end"),
        }
    }
}

impl From<Row> for ledger_line::ActiveModel {
    fn from(value: Row) -> Self {
        let journal_ref =
            JournalTransactionId::new(value.get("journal_id"), value.get("timestamp"));
        let mut lxtc: String = value.get("ledger_transaction_type_code");
        if lxtc.len() > ledger_xact_type_code::LEN {
            lxtc.truncate(ledger_xact_type_code::LEN);
        }
        let lxtc = LedgerXactTypeCode::from(lxtc);
        Self {
            ledger_id: value.get("ledger_id"),
            timestamp: value.get("timestamp"),
            ledger_xact_type_code: lxtc,
            amount: value.get("amount"),
            journal_ref,
        }
    }
}

impl From<Row> for ledger_transaction::ActiveModel {
    fn from(value: Row) -> Self {
        Self {
            ledger_id: value.get("ledger_id"),
            timestamp: value.get("timestamp"),
            ledger_dr_id: value.get("ledger_dr_id"),
        }
    }
}

impl From<Row> for ledger_xact_type::ActiveModel {
    fn from(value: Row) -> Self {
        let mut code: String = value.get("code");
        if code.len() > ledger_xact_type_code::LEN {
            code.truncate(ledger_xact_type_code::LEN);
        }
        let code = LedgerXactTypeCode::from(code);

        Self { code }
    }
}

impl PgStore {
    pub async fn build(connection_str: &str) -> Result<PgStore, OrmError> {
        let res = Inner::build(connection_str).await?;

        Ok(Self {
            inner: Arc::new(res),
        })
    }

    pub async fn insert_general_ledger(
        &self,
        model: &general_ledger::Model,
        root_model: &ledger::ActiveModel,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, name, root, currency_code) VALUES($1, $2, $3, $4) RETURNING *",
            general_ledger::ActiveModel::NAME
        );
        let gl = conn
            .query_one(
                query.as_str(),
                &[
                    &GeneralLedgerId::new(),
                    &model.name.as_str(),
                    &root_model.id,
                    &model.currency_code,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(general_ledger::ActiveModel::from(gl))
    }

    pub async fn find_general_ledger<'a>(
        &self,
        ids: Option<Vec<GeneralLedgerId>>,
    ) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1);",
            general_ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {};", general_ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&vec_id2uuid(ids)]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<general_ledger::ActiveModel>::new();
        for row in rows {
            let am = general_ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn update_general_ledger(
        &self,
        ids: Vec<GeneralLedgerId>,
        active_model: &general_ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "UPDATE {} SET root = $1 WHERE id = any ($2);",
            general_ledger::ActiveModel::NAME
        );

        let mut uuids = Vec::<uuid::Uuid>::new();
        for i in ids {
            uuids.push(*i)
        }

        conn.execute(query.as_str(), &[&active_model.root, &uuids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    pub async fn insert_ledger(
        &self,
        model: &ledger::Model,
    ) -> Result<ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, ledger_no, name, ledger_type, parent_id, currency_code) VALUES($1, $2, $3, $4, $5, $6) RETURNING *;",
            ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                query.as_str(),
                &[
                    &AccountId::new(),
                    &model.ledger_no.as_str(),
                    &model.name.as_str(),
                    &model.ledger_type,
                    &model.parent_id,
                    &model.currency_code,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger::ActiveModel::from(res))
    }

    async fn insert_ledger_intermediate(
        &self,
        model: &ledger_intermediate::Model,
    ) -> Result<ledger_intermediate::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id) VALUES($1) RETURNING *",
            ledger_intermediate::ActiveModel::NAME
        );
        let st = SqlData {
            statement: &query,
            params: &[&model.id],
        };
        let res = conn
            .query_one(st.statement, st.params)
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_intermediate::ActiveModel::from(res))
    }

    async fn insert_ledger_leaf(
        &self,
        model: &ledger_leaf::Model,
    ) -> Result<ledger_leaf::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id) VALUES($1) RETURNING *",
            ledger_leaf::ActiveModel::NAME
        );
        let st = SqlData {
            statement: &query,
            params: &[&model.id],
        };
        let res = conn
            .query_one(st.statement, st.params)
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_leaf::ActiveModel::from(res))
    }

    pub async fn find_ledger(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1)",
            ledger::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&vec_id2uuid(ids)]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::ActiveModel>::new();
        for row in rows {
            let am = ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_ledger_by_model(
        &self,
        model: &ledger::Model,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        let sql = format!(
            "SELECT * FROM {} WHERE ledger_no=$1",
            ledger::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(sql.as_str(), &[&model.ledger_no])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger::ActiveModel>::new();
        for row in rows {
            let am = ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn _find_ledger_intermediate<'a>(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger_intermediate::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id = any ($1)",
            ledger_intermediate::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_intermediate::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger_intermediate::ActiveModel>::new();
        for row in rows {
            let am = ledger_intermediate::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn _find_ledger_leaf<'a>(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger_leaf::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in ($1)",
            ledger_leaf::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_leaf::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger_leaf::ActiveModel>::new();
        for row in rows {
            let am = ledger_leaf::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn _find_ledger_derived<'a>(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger_derived::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in ($1)",
            ledger_derived::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_derived::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<ledger_derived::ActiveModel>::new();
        for row in rows {
            let am = ledger_derived::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn insert_journal(
        &self,
        model: &journal::Model,
    ) -> Result<journal::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, name, code) VALUES($1, $2, $3) RETURNING *",
            journal::ActiveModel::NAME
        );
        let st = SqlData {
            statement: &query,
            params: &[&JournalId::new(), &model.name, &model.code],
        };
        let res = conn
            .query_one(st.statement, st.params)
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal::ActiveModel::from(res))
    }

    pub async fn find_journal<'a>(
        &self,
        ids: Option<Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            journal::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", journal::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<journal::ActiveModel>::new();
        for row in rows {
            let am = journal::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_journal_by_code<'a>(
        &self,
        journal_code: &str,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        let statement = format!("SELECT * FROM {} WHERE code=$1", journal::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows = conn
            .query(statement.as_str(), &[&journal_code])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<journal::ActiveModel>::new();
        for row in rows {
            let am = journal::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn insert_journal_transaction(
        &self,
        model: &journal_transaction_record::Model,
    ) -> Result<journal_transaction_record::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(journal_id, timestamp, explanation) 
                VALUES($1, $2, $3) RETURNING *",
            journal_transaction_record::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.journal_id, &model.timestamp, &model.explanation],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal_transaction_record::ActiveModel::from(res))
    }

    async fn insert_journal_transaction_line_ledger(
        &self,
        model: &journal_transaction_line_ledger::Model,
    ) -> Result<journal_transaction_line_ledger::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp, ledger_id, xact_type, amount, state) 
                    VALUES($1, $2, $3, $4, $5, $6) RETURNING *",
            journal_transaction_line_ledger::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.ledger_id,
                    &model.xact_type,
                    &model.amount,
                    &model.state,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal_transaction_line_ledger::ActiveModel::from(res))
    }

    async fn insert_journal_transaction_line_account(
        &self,
        model: &journal_transaction_line_account::Model,
    ) -> Result<journal_transaction_line_account::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(journal_id, timestamp, account_id, xact_type, amount, state) 
                    VALUES($1, $2, $3, $4, $5, $6) RETURNING *",
            journal_transaction_line_account::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.journal_id,
                    &model.timestamp,
                    &model.account_id,
                    &model.xact_type,
                    &model.amount,
                    &model.state,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(journal_transaction_line_account::ActiveModel::from(res))
    }

    async fn _update_journal_transaction_line_ledger(
        &self,
        model: &journal_transaction_line_ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "UPDATE {} 
                SET ledger_id=$1, xact_type=$2, amount=$3, state=$4
                    WHERE journal_id=$5::JournalId AND timestamp=$6",
            journal_transaction_line_ledger::ActiveModel::NAME
        );
        let res = conn
            .execute(
                sql.as_str(),
                &[
                    &model.ledger_id,
                    &model.xact_type,
                    &model.amount,
                    &model.state,
                    &model.journal_id,
                    &model.timestamp,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(res)
    }

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal_transaction_line_ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "UPDATE {} 
                SET state=$1, posting_ref=$2
                    WHERE journal_id=$3::JournalId AND timestamp=$4 and ledger_id=$5::AccountId",
            journal_transaction_line_ledger::ActiveModel::NAME
        );
        let res = conn
            .execute(
                sql.as_str(),
                &[
                    &TransactionState::Posted,
                    &line.posting_ref,
                    &id.journal_id(),
                    &id.timestamp(),
                    &line.ledger_id,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(res)
    }

    pub async fn find_journal_transaction_record<'a>(
        &self,
        ids: &Option<Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction_record::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_record::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal_transaction_record::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.journal_id(), &id.timestamp()])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<journal_transaction_record::ActiveModel>::new();
        for row in rows {
            let am = journal_transaction_record::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_journal_transaction_line_ledger<'a>(
        &self,
        ids: &Option<Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction_line_ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_line_ledger::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal_transaction_line_ledger::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.journal_id(), &id.timestamp()])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<journal_transaction_line_ledger::ActiveModel>::new();
        for row in rows {
            let am = journal_transaction_line_ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_journal_transaction_line_account<'a>(
        &self,
        ids: &Option<Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction_line_account::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE journal_id=$1::JournalId AND timestamp=$2",
            journal_transaction_line_account::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            journal_transaction_line_account::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.journal_id(), &id.timestamp()])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<journal_transaction_line_account::ActiveModel>::new();
        for row in rows {
            let am = journal_transaction_line_account::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn _insert_journal_entry(
        &self,
        model: &journal_entry::Model,
    ) -> Result<journal_entry::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO 
                {}(ledger_id, timestamp, ledger_xact_type_code, journal_id, amount) 
                    VALUES($1, $2, $3, $4, $5) RETURNING *",
            ledger_line::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.ledger_id,
                    &model.timestamp,
                    &LedgerXactTypeCode::from_str("LL").unwrap(),
                    &model.journal_ref.journal_id(),
                    &model.amount,
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;
        let res = ledger_line::ActiveModel::from(res);

        Ok(journal_entry::ActiveModel::from(res))
    }

    pub async fn insert_ledger_line(
        &self,
        model: &ledger_line::Model,
    ) -> Result<ledger_line::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(ledger_id, timestamp, ledger_transaction_type_code, amount, journal_id)
                VALUES($1, $2, $3, $4, $5) RETURNING *",
            ledger_line::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[
                    &model.ledger_id,
                    &model.timestamp,
                    &model.ledger_xact_type_code,
                    &model.amount,
                    &model.journal_ref.journal_id(),
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_line::ActiveModel::from(res))
    }

    pub async fn insert_ledger_transaction(
        &self,
        model: &ledger_transaction::Model,
    ) -> Result<ledger_transaction::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "INSERT INTO {}(ledger_id, timestamp, ledger_dr_id)
                VALUES($1, $2, $3) RETURNING *",
            ledger_transaction::ActiveModel::NAME
        );
        let res = conn
            .query_one(
                sql.as_str(),
                &[&model.ledger_id, &model.timestamp, &model.ledger_dr_id],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_transaction::ActiveModel::from(res))
    }

    pub async fn find_ledger_line(
        &self,
        ids: &Option<Vec<LedgerKey>>,
    ) -> Result<Vec<ledger_line::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id=$1::AccountId AND timestamp=$2",
            ledger_line::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_line::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.ledger_id, &id.timestamp])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<ledger_line::ActiveModel>::new();
        for row in rows {
            let am = ledger_line::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_ledger_line_by_id(
        &self,
        ledger_ids: &Vec<AccountId>,
    ) -> Result<Vec<ledger_line::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id = any ($1::AccountId[])",
            ledger_line::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(search_one.as_str(), &[ledger_ids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        let mut records = Vec::<ledger_line::ActiveModel>::new();
        for row in rows {
            let am = ledger_line::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_ledger_transaction(
        &self,
        ids: &Option<Vec<LedgerKey>>,
    ) -> Result<Vec<ledger_transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id=$1::AccountId AND timestamp=$2",
            ledger_transaction::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", ledger_transaction::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let rows: Vec<Row> = match ids {
            Some(ids) => {
                let mut temp_ids = Vec::<tokio_postgres::Row>::new();
                for id in ids {
                    let mut res = conn
                        .query(search_one.as_str(), &[&id.ledger_id, &id.timestamp])
                        .await
                        .map_err(|e| OrmError::Internal(e.to_string()))?;
                    if !res.is_empty() {
                        temp_ids.append(&mut res);
                    }
                }

                temp_ids
            }
            None => conn
                .query(search_all.as_str(), &[])
                .await
                .map_err(|e| OrmError::Internal(e.to_string()))?,
        };
        let mut records = Vec::<ledger_transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger_transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_ledger_transaction_by_id(
        &self,
        ledger_ids: &Vec<AccountId>,
    ) -> Result<Vec<ledger_transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id = any ($1::AccountId[])",
            ledger_transaction::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(search_one.as_str(), &[&ledger_ids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        let mut records = Vec::<ledger_transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger_transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_ledger_transaction_by_dr(
        &self,
        ledger_ids: &Vec<AccountId>,
    ) -> Result<Vec<ledger_transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_dr_id = any ($1::AccountId[])",
            ledger_transaction::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(search_one.as_str(), &[&ledger_ids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        let mut records = Vec::<ledger_transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger_transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn journal_entry_by_posting_ref(
        &self,
        posting_ref: PostingRef,
    ) -> Result<Option<journal_entry::ActiveModel>, OrmError> {
        let entry = self.find_ledger_line(&Some(vec![posting_ref.key])).await?;
        for e in entry.iter() {
            if e.ledger_id == posting_ref.account_id {
                return Ok(Some(journal_entry::ActiveModel {
                    ledger_id: e.ledger_id,
                    timestamp: e.timestamp,
                    xact_type: XactType::Cr,
                    amount: e.amount,
                    journal_ref: e.journal_ref,
                }));
            }
        }
        let xact = self
            .find_ledger_transaction(&Some(vec![posting_ref.key]))
            .await?;
        for t in xact {
            if t.ledger_dr_id == posting_ref.account_id {
                let counterpart = self
                    .ledger_line_by_key(LedgerKey {
                        ledger_id: t.ledger_id,
                        timestamp: t.timestamp,
                    })
                    .await
                    .unwrap();
                return Ok(Some(journal_entry::ActiveModel {
                    ledger_id: t.ledger_dr_id,
                    timestamp: t.timestamp,
                    xact_type: XactType::Dr,
                    amount: counterpart.amount,
                    journal_ref: counterpart.journal_ref,
                }));
            }
        }

        Ok(None)
    }

    pub async fn journal_entries_by_account_id(
        &self,
        account_id: AccountId,
    ) -> Vec<journal_entry::ActiveModel> {
        let mut res = Vec::<journal_entry::ActiveModel>::new();
        let entries = self.ledger_line_by_id(account_id).await;
        let xacts = self.ledger_transaction_by_dr(account_id).await;
        for e in entries {
            res.push(journal_entry::ActiveModel {
                ledger_id: e.ledger_id,
                timestamp: e.timestamp,
                xact_type: XactType::Cr,
                amount: e.amount,
                journal_ref: e.journal_ref,
            })
        }
        for t in xacts {
            let key = LedgerKey {
                ledger_id: t.ledger_id,
                timestamp: t.timestamp,
            };
            let counterpart = self.ledger_line_by_key(key).await;
            if let Some(counterpart) = counterpart {
                res.push(journal_entry::ActiveModel {
                    ledger_id: t.ledger_dr_id,
                    timestamp: t.timestamp,
                    xact_type: XactType::Dr,
                    amount: counterpart.amount,
                    journal_ref: counterpart.journal_ref,
                });
            } else {
                // TODO: Log error
                eprintln!("failed to find counterpart journal entry: {key}");
            }
        }

        res
    }

    async fn ledger_line_by_id(&self, account_id: AccountId) -> Vec<ledger_line::ActiveModel> {
        let res = self.find_ledger_line_by_id(&vec![account_id]).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("find_ledger_line_by_id failed: {}", res);

            return Vec::<ledger_line::ActiveModel>::new();
        }

        res.unwrap()
    }

    async fn ledger_line_by_key(&self, key: LedgerKey) -> Option<ledger_line::ActiveModel> {
        let res = self.find_ledger_line(&Some(vec![key])).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("ledger_line_by_key failed: {res}");

            return None;
        }

        Some(res.unwrap()[0])
    }

    pub async fn ledger_transaction_by_id(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger_transaction::ActiveModel> {
        let res = self.find_ledger_transaction_by_id(&vec![account_id]).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("ledger_transaction_by_id failed: {res}");

            return Vec::<ledger_transaction::ActiveModel>::new();
        }

        res.unwrap()
    }

    pub async fn ledger_transaction_by_dr(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger_transaction::ActiveModel> {
        let res = self.find_ledger_transaction_by_dr(&vec![account_id]).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("ledger_transaction_by_dr failed: {res}");

            return Vec::<ledger_transaction::ActiveModel>::new();
        }

        res.unwrap()
    }

    pub async fn post_journal_transaction(
        &self,
        jxact_id: JournalTransactionId,
    ) -> Result<bool, OrmError> {
        let ledger_xact_type = self.get_journal_entry_type(jxact_id).await?;

        let mut jxact_lines = self
            .find_journal_transaction_line_ledger(&Some(vec![jxact_id]))
            .await?;
        let cr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Cr)
            .collect::<Vec<_>>();
        let dr_xact_lines = jxact_lines
            .iter()
            .filter(|am| am.xact_type == XactType::Dr)
            .collect::<Vec<_>>();
        let mut ledger_posted_list = Vec::<journal_transaction_line_ledger::ActiveModel>::new();
        for (cr, dr) in zip(cr_xact_lines.clone(), dr_xact_lines.clone()) {
            let key = LedgerKey {
                ledger_id: cr.ledger_id,
                timestamp: cr.timestamp,
            };
            let entry = ledger_line::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_xact_type_code: ledger_xact_type.code,
                amount: cr.amount,
                journal_ref: jxact_id,
            };
            let tx_dr = ledger_transaction::Model {
                ledger_id: key.ledger_id,
                timestamp: key.timestamp,
                ledger_dr_id: dr.ledger_id,
            };

            let _ = self.insert_ledger_line(&entry).await?;
            let _ = self.insert_ledger_transaction(&tx_dr).await?;
            let mut cr = *cr;
            cr.state = TransactionState::Posted;
            cr.posting_ref = Some(PostingRef {
                key,
                account_id: cr.ledger_id,
            });
            let mut dr = *dr;
            dr.state = TransactionState::Posted;
            dr.posting_ref = Some(PostingRef {
                key,
                account_id: dr.ledger_id,
            });
            ledger_posted_list.push(dr);
            ledger_posted_list.push(cr);
        }

        for line in jxact_lines.iter_mut() {
            for post_line in ledger_posted_list.iter() {
                if line.id() == post_line.id() {
                    self.update_journal_transaction_line_ledger_posting_ref(jxact_id, post_line)
                        .await?;
                }
            }
        }

        Ok(true)
    }

    async fn get_journal_entry_type(
        &self,
        _jxact_id: JournalTransactionId,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        let ll_code = LedgerXactTypeCode::from_str(ledger_xact_type_code::XACT_LEDGER).unwrap();
        let conn = self.get_connection().await?;
        let sql = format!(
            "SELECT code FROM {} WHERE code = $1",
            ledger_xact_type::ActiveModel::NAME
        );
        let res = conn
            .query_one(sql.as_str(), &[&ll_code.as_str()])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(ledger_xact_type::ActiveModel::from(res))
    }

    pub async fn insert_period(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, fiscal_year, period_start, period_end) VALUES($1, $2, $3, $4) RETURNING *",
            accounting_period::ActiveModel::NAME
        );
        let st = SqlData {
            statement: &query,
            params: &[
                &PeriodId::new(),
                &model.fiscal_year,
                &model.period_start,
                &(model.period_start + RelativeDuration::years(1) + RelativeDuration::days(-1)),
            ],
        };
        let res = conn
            .query_one(st.statement, st.params)
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(accounting_period::ActiveModel::from(res))
    }

    pub async fn find_period<'a>(
        &self,
        ids: Option<Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            accounting_period::ActiveModel::NAME
        );
        let search_all = format!("SELECT * FROM {}", accounting_period::ActiveModel::NAME);
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<accounting_period::ActiveModel>::new();
        for row in rows {
            let am = accounting_period::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn find_period_by_year(
        &self,
        model: &accounting_period::Model,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        let sql = format!(
            "SELECT * FROM {} WHERE fiscal_year=$1",
            accounting_period::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(sql.as_str(), &[&model.fiscal_year])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<accounting_period::ActiveModel>::new();
        for row in rows {
            let am = accounting_period::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    pub async fn insert_interim_period(
        &self,
        model: &interim_accounting_period::Model,
    ) -> Result<interim_accounting_period::ActiveModel, OrmError> {
        let conn = self.get_connection().await?;
        let query = format!(
            "INSERT INTO {}(id, parent_id, interim_start, interim_end) VALUES($1, $2, $3, $4) RETURNING *",
            interim_accounting_period::ActiveModel::NAME
        );
        let st = SqlData {
            statement: &query,
            params: &[
                &InterimPeriodId::new(),
                &model.parent_id,
                &model.start,
                &model.end,
            ],
        };
        let res = conn
            .query_one(st.statement, st.params)
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(interim_accounting_period::ActiveModel::from(res))
    }

    pub async fn find_interim_period(
        &self,
        ids: Option<Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_accounting_period::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE id in $1",
            interim_accounting_period::ActiveModel::NAME
        );
        let search_all = format!(
            "SELECT * FROM {}",
            interim_accounting_period::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let qry = match ids {
            Some(ids) => conn.query(search_one.as_str(), &[&ids]).await,
            None => conn.query(search_all.as_str(), &[]).await,
        };
        let rows = qry.map_err(|e| OrmError::Internal(e.to_string()))?;
        let mut records = Vec::<interim_accounting_period::ActiveModel>::new();
        for row in rows {
            let am = interim_accounting_period::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn get_connection(&self) -> Result<PgConn, OrmError> {
        self.inner.get_connection().await
    }
}

impl Inner {
    pub async fn build(connection_str: &str) -> Result<Inner, OrmError> {
        let pool = Self::connect(connection_str).await?;

        Ok(Self { pool })
    }

    async fn connect(connection_str: &str) -> Result<PgPool, OrmError> {
        let config =
            Config::from_str(connection_str).map_err(|e| OrmError::Internal(e.to_string()))?;
        let manager = PgConnectionManager::new(config, NoTls);

        Ok(Pool::builder()
            .max_open(MAX_OPEN_CONNECTIONS)
            .max_idle(MAX_IDLE_CONNECTIONS)
            .get_timeout(Some(Duration::from_secs(MAX_TIMEOUT_SECONDS)))
            .build(manager))
    }

    async fn get_connection(&self) -> Result<PgConn, OrmError> {
        self.pool
            .get()
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }
}

#[async_trait]
impl AccountRepository<accounting_period::Model, accounting_period::ActiveModel, PeriodId>
    for PgStore
{
    async fn create(
        &self,
        model: &accounting_period::Model,
    ) -> Result<accounting_period::ActiveModel, OrmError> {
        let periods = self.find_period_by_year(model).await?;
        if periods.is_empty() {
            let active_model = self.insert_period(model).await?;

            let _ = match model.period_type {
                InterimType::CalendarMonth => active_model.create_interim_calendar(self).await,
                InterimType::FourWeek => todo!(),
                InterimType::FourFourFiveWeek => todo!(),
            }
            .map_err(OrmError::Internal)?;

            return Ok(active_model);
        }

        Err(OrmError::DuplicateRecord(
            "duplicate accounting period".into(),
        ))
    }

    async fn search(
        &self,
        ids: Option<Vec<PeriodId>>,
    ) -> Result<Vec<accounting_period::ActiveModel>, OrmError> {
        Ok(self.find_period(ids).await?)
    }

    async fn update(
        &self,
        _ids: &[PeriodId],
        _model: &accounting_period::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl
    AccountRepository<
        interim_accounting_period::Model,
        interim_accounting_period::ActiveModel,
        InterimPeriodId,
    > for PgStore
{
    async fn create(
        &self,
        model: &interim_accounting_period::Model,
    ) -> Result<interim_accounting_period::ActiveModel, OrmError> {
        Ok(self.insert_interim_period(model).await?)
    }

    async fn search(
        &self,
        ids: Option<Vec<InterimPeriodId>>,
    ) -> Result<Vec<interim_accounting_period::ActiveModel>, OrmError> {
        let mut res = self.find_interim_period(ids).await?;
        res.sort_by(|a, b| a.start.cmp(&b.start));

        Ok(res)
    }

    async fn update(
        &self,
        _ids: &[InterimPeriodId],
        _model: &interim_accounting_period::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<general_ledger::Model, general_ledger::ActiveModel, GeneralLedgerId>
    for PgStore
{
    async fn create(
        &self,
        model: &general_ledger::Model,
    ) -> Result<general_ledger::ActiveModel, OrmError> {
        let root = self
            .find_ledger_by_model(&ledger::Model {
                ledger_no: ArrayShortString::from_str("0").unwrap(),
                ledger_type: LedgerType::Intermediate,
                name: ArrayLongString::from_str("Root").unwrap(),
                parent_id: None,
                currency_code: None,
            })
            .await?;
        let gl = self.insert_general_ledger(model, &root[0]).await?;

        Ok(gl)
    }

    async fn search(
        &self,
        ids: Option<Vec<GeneralLedgerId>>,
    ) -> Result<Vec<general_ledger::ActiveModel>, OrmError> {
        Ok(self.find_general_ledger(ids).await?)
    }

    async fn update(
        &self,
        _ids: &[GeneralLedgerId],
        _active_model: &general_ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
        // Ok(self.update_general_ledger(ids, active_model).await?)
    }
}

#[async_trait]
impl AccountRepository<ledger::Model, ledger::ActiveModel, AccountId> for PgStore {
    async fn create(&self, model: &ledger::Model) -> Result<ledger::ActiveModel, OrmError> {
        let parent = match model.parent_id {
            Some(id) => self.search(Some(vec![id])).await?,
            None => return Err(OrmError::Constraint("ledger must have parent".into())),
        };
        if parent[0].ledger_type != LedgerType::Intermediate {
            return Err(OrmError::Validation(
                "parent ledger is not an Intermediate Ledger".into(),
            ));
        }

        if model.ledger_no != ArrayShortString::from_str("0").unwrap()
            && !self.find_ledger_by_model(model).await?.is_empty()
        {
            return Err(OrmError::DuplicateRecord(format!(
                "duplicate ledger number: {}",
                model.ledger_no
            )));
        }
        let ledger = self.insert_ledger(model).await?;
        if model.ledger_type == LedgerType::Intermediate {
            let intermediate = ledger_intermediate::Model { id: ledger.id };
            let _ = self.insert_ledger_intermediate(&intermediate).await?;
        } else {
            let account = ledger_leaf::Model { id: ledger.id };
            let _ = self.insert_ledger_leaf(&account).await?;
        }

        Ok(ledger)
    }

    async fn search(
        &self,
        ids: Option<Vec<AccountId>>,
    ) -> Result<Vec<ledger::ActiveModel>, OrmError> {
        Ok(self.find_ledger(ids).await?)
    }

    async fn update(
        &self,
        _ids: &[AccountId],
        _model: &ledger::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository<journal::Model, journal::ActiveModel, JournalId> for PgStore {
    async fn create(&self, model: &journal::Model) -> Result<journal::ActiveModel, OrmError> {
        let id = JournalId::new();
        let journal = journal::ActiveModel {
            id,
            name: model.name.clone(),
            code: model.code.clone(),
        };
        let duplicates = self.find_journal_by_code(&model.code).await?;
        if !duplicates.is_empty() {
            return Err(OrmError::DuplicateRecord(
                "duplicate Journal Id or Code".into(),
            ));
        }
        self.insert_journal(model).await?;

        Ok(journal)
    }

    async fn search(
        &self,
        ids: Option<Vec<JournalId>>,
    ) -> Result<Vec<journal::ActiveModel>, OrmError> {
        Ok(self.find_journal(ids).await?)
    }

    async fn update(
        &self,
        _ids: &[JournalId],
        _model: &journal::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}

#[async_trait]
impl
    AccountRepository<
        journal_transaction::Model,
        journal_transaction::ActiveModel,
        JournalTransactionId,
    > for PgStore
{
    async fn create(
        &self,
        model: &journal_transaction::Model,
    ) -> Result<journal_transaction::ActiveModel, OrmError> {
        let jtx_id = JournalTransactionId::new(model.journal_id, model.timestamp);
        for line in model.lines.iter() {
            if line.ledger_id.is_none() && line.account_id.is_none() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() && line.account_id.is_some() {
                return Err(OrmError::Internal(format!(
                    "both ledger and account fields NOT empty: transaction: {}",
                    jtx_id
                )));
            } else if line.ledger_id.is_some() {
                if self
                    .search(Some(vec![line.ledger_id.unwrap()]))
                    .await?
                    .is_empty()
                {
                    return Err(OrmError::RecordNotFound(format!(
                        "account id: {}",
                        line.ledger_id.unwrap()
                    )));
                }
            } else if line.account_id.is_some() {
                //     if self
                //         .search_account(Some(vec![line.account_id.unwrap()]))
                //         .await?
                //         .is_empty()
                //     {
                //         return Err(OrmError::RecordNotFound(format!(
                //             "account id: {}",
                //             line.ledger_id.unwrap()
                //         )));
                //     }
            }
        }

        let jtx = journal_transaction_record::Model {
            journal_id: model.journal_id,
            timestamp: model.timestamp,
            explanation: model.explanation,
        };
        let record = self.insert_journal_transaction(&jtx).await?;

        let mut res_tx_lines = Vec::<journal_transaction_line::ActiveModel>::new();
        for line in model.lines.iter() {
            if line.ledger_id.is_some() {
                let jtx_line = journal_transaction_line_ledger::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    ledger_id: line.ledger_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = self
                    .insert_journal_transaction_line_ledger(&jtx_line)
                    .await?;
                res_tx_lines.push(journal_transaction_line::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: Some(jtx_line.ledger_id),
                    account_id: None,
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            } else {
                let jtx_line = journal_transaction_line_account::Model {
                    journal_id: model.journal_id,
                    timestamp: model.timestamp,
                    state: TransactionState::Pending,
                    account_id: line.account_id.unwrap(),
                    xact_type: line.xact_type,
                    amount: line.amount,
                    posting_ref: None,
                };
                let jtx_line = self
                    .insert_journal_transaction_line_account(&jtx_line)
                    .await?;
                res_tx_lines.push(journal_transaction_line::ActiveModel {
                    journal_id: jtx_line.journal_id,
                    timestamp: jtx_line.timestamp,
                    ledger_id: None,
                    account_id: Some(jtx_line.account_id),
                    xact_type: jtx_line.xact_type,
                    amount: jtx_line.amount,
                    posting_ref: jtx_line.posting_ref,
                    state: jtx_line.state,
                })
            }
        }

        Ok(journal_transaction::ActiveModel {
            journal_id: record.journal_id,
            timestamp: record.timestamp,
            explanation: record.explanation,
            lines: res_tx_lines,
        })
    }

    async fn search(
        &self,
        ids: Option<Vec<JournalTransactionId>>,
    ) -> Result<Vec<journal_transaction::ActiveModel>, OrmError> {
        let xacts = self.find_journal_transaction_record(&ids).await?;
        let record_lines = self.find_journal_transaction_line_ledger(&ids).await?;

        if !xacts.is_empty() {
            let mut lines = Vec::<journal_transaction_line::ActiveModel>::new();
            for r in record_lines {
                lines.push(journal_transaction_line::ActiveModel {
                    journal_id: r.journal_id,
                    timestamp: r.timestamp,
                    ledger_id: Some(r.ledger_id),
                    account_id: None,
                    xact_type: r.xact_type,
                    amount: r.amount,
                    state: r.state,
                    posting_ref: r.posting_ref,
                })
            }

            return Ok(vec![journal_transaction::ActiveModel {
                journal_id: xacts[0].journal_id,
                timestamp: xacts[0].timestamp,
                explanation: xacts[0].explanation,
                lines,
            }]);
        }

        Ok(Vec::<journal_transaction::ActiveModel>::new())
    }

    async fn update(
        &self,
        _ids: &[JournalTransactionId],
        _model: &journal_transaction::ActiveModel,
    ) -> Result<u64, OrmError> {
        todo!()
    }
}
