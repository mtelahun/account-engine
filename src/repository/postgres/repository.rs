use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use mobc::{Connection, Pool};
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{Config, NoTls, Row};

use crate::domain::{ledger_xact_type_code, AccountId, JournalTransactionId, LedgerXactTypeCode};
use crate::entity::{
    accounting_period, journal_transaction_line_ledger, ledger, ledger_line, ledger_transaction,
    ledger_xact_type, LedgerKey, TransactionState,
};
use crate::orm::{OrmError, Repository, Resource, ResourceOperations};

const MAX_OPEN_CONNECTIONS: u64 = 32;
const MAX_IDLE_CONNECTIONS: u64 = 8;
const MAX_TIMEOUT_SECONDS: u64 = 15;

type PgConn = Connection<PgConnectionManager<NoTls>>;
type PgPool = Pool<PgConnectionManager<NoTls>>;

pub struct PostgresRepository {
    uri: String,
    name: String,
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(name: &str, uri: &str) -> Result<Self, OrmError> {
        let full_uri = format!("{uri}/{name}");
        let config = Config::from_str(&full_uri).map_err(|e| OrmError::Internal(e.to_string()))?;
        let manager = PgConnectionManager::new(config, NoTls);

        let pool = Pool::builder()
            .max_open(MAX_OPEN_CONNECTIONS)
            .max_idle(MAX_IDLE_CONNECTIONS)
            .get_timeout(Some(Duration::from_secs(MAX_TIMEOUT_SECONDS)))
            .build(manager);

        Ok(Self {
            name: name.to_string(),
            uri: uri.to_string(),
            pool,
        })
    }

    pub async fn new_schema(name: &str, uri: &str) -> Result<Self, OrmError> {
        Self::create_db(name, uri).await;

        Self::new(name, uri)
    }

    async fn create_db(name: &str, uri: &str) {
        let pool = sqlx::PgPool::connect(uri)
            .await
            .expect("sqlx failed to connect to PostgreSQL");
        let sql = format!(r#"CREATE DATABASE "{name}";"#);
        sqlx::query(&sql)
            .execute(&pool)
            .await
            .expect("failed to create the database");

        Self::migrate_db(name, uri).await;
    }

    async fn migrate_db(name: &str, uri: &str) {
        let pool = sqlx::PgPool::connect(&format!("{uri}/{name}"))
            .await
            .expect("failed to connect to newly created database");
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("database migration failed");
    }

    pub(crate) async fn get_connection(&self) -> Result<PgConn, OrmError> {
        self.pool
            .get()
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))
    }

    pub(crate) async fn find_ledger_by_model(
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

    pub(crate) async fn find_period_by_year(
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

    pub(crate) async fn ledger_line_by_key(
        &self,
        key: LedgerKey,
    ) -> Option<ledger_line::ActiveModel> {
        let res = self.find_ledger_line(&Some(vec![key])).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("ledger_line_by_key failed: {res}");

            return None;
        }

        Some(res.unwrap()[0])
    }

    pub(crate) async fn ledger_line_by_id(
        &self,
        account_id: AccountId,
    ) -> Vec<ledger_line::ActiveModel> {
        let res = self.find_ledger_line_by_id(&vec![account_id]).await;
        if let Err(res) = res {
            // TODO: Log error
            eprintln!("find_ledger_line_by_id failed: {}", res);

            return Vec::<ledger_line::ActiveModel>::new();
        }

        res.unwrap()
    }

    pub(crate) async fn ledger_transaction_by_dr(
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

    pub(crate) async fn find_ledger_transaction_by_dr(
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

    pub(crate) async fn get_journal_entry_type(
        &self,
        _jxact_id: JournalTransactionId,
    ) -> Result<ledger_xact_type::ActiveModel, OrmError> {
        let ll_code = LedgerXactTypeCode::from_str(ledger_xact_type_code::XACT_LEDGER).unwrap();

        Ok(self.get(Some(&vec![ll_code])).await?[0])
    }

    pub(crate) async fn update_journal_transaction_line_ledger_posting_ref(
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
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn create_schema(&self) -> Result<(), OrmError> {
        Self::migrate_db(&self.name, &self.uri).await;

        Ok(())
    }
}
