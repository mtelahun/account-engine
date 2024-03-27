#![allow(clippy::diverging_sub_expression)]
use std::str::FromStr;
use std::time::Duration;

use async_trait::async_trait;
use mobc::{Connection, Pool};
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{Config, NoTls};

use crate::domain::{
    ArrayString24, JournalTransactionId, LedgerId, Sequence, SpecialJournalTemplateId,
};
use crate::resource::{
    accounting_period, journal,
    ledger::{self, transaction},
    organization, TransactionState,
};
use crate::store::{OrmError, Resource};
use crate::Store;

const MAX_OPEN_CONNECTIONS: u64 = 32;
const MAX_IDLE_CONNECTIONS: u64 = 8;
const MAX_TIMEOUT_SECONDS: u64 = 15;

type PgConn = Connection<PgConnectionManager<NoTls>>;
type PgPool = Pool<PgConnectionManager<NoTls>>;

pub struct PostgresStore {
    uri: String,
    name: String,
    pool: PgPool,
}

impl PostgresStore {
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
}

#[async_trait]
impl Store for PostgresStore {
    async fn create_schema(&self) -> Result<(), OrmError> {
        Self::migrate_db(&self.name, &self.uri).await;

        Ok(())
    }

    async fn organization(&self) -> Result<organization::ActiveModel, OrmError> {
        todo!()
    }

    async fn update_journal_transaction_line_ledger_posting_ref(
        &self,
        id: JournalTransactionId,
        line: &journal::transaction::general::line::ActiveModel,
    ) -> Result<u64, OrmError> {
        let conn = self.get_connection().await?;
        let sql = format!(
            "UPDATE {} 
                SET state=$1, dr_posting_ref=$2, cr_posting_ref=$3
                    WHERE journal_id=$4::JournalId AND timestamp=$5",
            journal::transaction::general::line::ActiveModel::NAME
        );
        let res = conn
            .execute(
                sql.as_str(),
                &[
                    &TransactionState::Posted,
                    &line.dr_posting_ref,
                    &line.cr_posting_ref,
                    &id.journal_id(),
                    &id.timestamp(),
                ],
            )
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        Ok(res)
    }

    async fn find_ledger_by_no(
        &self,
        no: ArrayString24,
    ) -> Result<Option<ledger::ActiveModel>, OrmError> {
        let sql = format!(
            "SELECT * FROM {} WHERE ledger_no=$1",
            ledger::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(sql.as_str(), &[&no])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        if rows.len() > 1 {
            return Err(OrmError::Validation(
                "found multiple ledgers with the same ledger number".into(),
            ));
        }
        if let Some(row) = rows.into_iter().next() {
            return Ok(Some(ledger::ActiveModel::from(row)));
        }

        Ok(None)
    }

    async fn journal_entries_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<ledger::transaction::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_id = any ($1::LedgerId[])",
            ledger::transaction::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(search_one.as_str(), &[&ids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        let mut records = Vec::<ledger::transaction::ActiveModel>::new();
        for row in rows {
            let am = ledger::transaction::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn journal_entry_ledgers_by_ledger(
        &self,
        ids: &[LedgerId],
    ) -> Result<Vec<transaction::ledger::ActiveModel>, OrmError> {
        let search_one = format!(
            "SELECT * FROM {} WHERE ledger_dr_id = any ($1::LedgerId[])",
            transaction::ledger::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(search_one.as_str(), &[&ids])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;

        let mut records = Vec::<transaction::ledger::ActiveModel>::new();
        for row in rows {
            let am = transaction::ledger::ActiveModel::from(row);
            records.push(am);
        }

        Ok(records)
    }

    async fn find_journal_by_code<'a>(
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

    async fn find_period_by_fiscal_year(
        &self,
        fy: i32,
    ) -> Result<Option<accounting_period::ActiveModel>, OrmError> {
        let sql = format!(
            "SELECT * FROM {} WHERE fiscal_year=$1",
            accounting_period::ActiveModel::NAME
        );
        let conn = self.get_connection().await?;
        let rows = conn
            .query(sql.as_str(), &[&fy])
            .await
            .map_err(|e| OrmError::Internal(e.to_string()))?;
        if rows.len() > 1 {
            return Err(OrmError::Validation(
                "found multiple accounting periods with the same fiscal year".into(),
            ));
        }
        if let Some(row) = rows.into_iter().next() {
            return Ok(Some(accounting_period::ActiveModel::from(row)));
        }

        Ok(None)
    }

    async fn get_journal_transaction_columns<'a>(
        &self,
        _ids: &'a [JournalTransactionId],
        _sequence: Sequence,
    ) -> Result<Vec<journal::transaction::special::column::ActiveModel>, OrmError> {
        todo!()
    }

    async fn get_journal_transaction_template_columns(
        &self,
        _id: SpecialJournalTemplateId,
    ) -> Result<Vec<journal::transaction::special::template::column::ActiveModel>, OrmError> {
        todo!()
    }
}
