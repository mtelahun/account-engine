use account_engine::{
    domain::{
        entity::{
            external_account::account_id::AccountId,
            subsidiary_ledger::external_xact_type_code::ExternalXactTypeCode,
        },
        journal_transaction::{JournalTransactionColumn, SpecialJournalTransaction},
        special_journal::special_journal_template_id::SpecialJournalTemplateId,
        ServiceError,
    },
    resource::{
        journal::{self, LedgerAccountPostingRef},
        ledger, LedgerKey,
    },
    shared_kernel::{ArrayString128, JournalId, JournalTransactionId, Sequence, XactType},
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[async_trait::async_trait]
pub trait ServiceTestInterface {
    // LedgerService
    //

    async fn journal_entry_by_key(
        &self,
        key: LedgerKey,
    ) -> Result<Option<ledger::transaction::ActiveModel>, ServiceError>;

    // SubsidiaryLedgerService
    //

    async fn get_journal_entry_transaction_account(
        &self,
        posting_ref: &LedgerAccountPostingRef,
    ) -> Result<ledger::transaction::account::ActiveModel, ServiceError>;

    // GeneralJournalService
    //

    async fn create_general_transaction(
        &self,
        model: &journal::transaction::general::Model,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError>;

    // SpecialJournalService
    //

    async fn create_subsidiary_transaction<'a>(
        &self,
        journal_id: &JournalId,
        timestamp: NaiveDateTime,
        tpl_id: &SpecialJournalTemplateId,
        account_id: AccountId,
        account_xact_type: XactType,
        xact_type_external_code: &ExternalXactTypeCode,
        amount: Decimal,
        explanation: &ArrayString128,
        tpl_col: &Vec<journal::transaction::special::template::column::ActiveModel>,
        line_models: &'a [JournalTransactionColumn],
    ) -> Result<
        (
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        ),
        ServiceError,
    >;

    async fn get_subsidiary_transactions(
        &self,
        ids: Option<&Vec<JournalTransactionId>>,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    >;

    async fn get_subsidiary_transactions_by_journal(
        &self,
        id: JournalId,
    ) -> Result<
        Vec<(
            SpecialJournalTransaction<journal::transaction::special::ActiveModel>,
            Vec<JournalTransactionColumn>,
        )>,
        ServiceError,
    >;

    async fn get_subsidiary_transaction_columns(
        &self,
        id: JournalTransactionId,
    ) -> Result<Vec<JournalTransactionColumn>, ServiceError>;

    // SpecialJournalTransactionService
    //

    async fn post_transaction(&self, id: JournalTransactionId) -> Result<bool, ServiceError>;

    async fn post_subsidiary_ledger(&self, id: JournalTransactionId) -> Result<bool, ServiceError>;

    async fn post_general_ledger(
        &self,
        journal_id: JournalId,
        ids: &Vec<JournalTransactionId>,
    ) -> Result<bool, ServiceError>;

    async fn get_column_total(
        &self,
        id: JournalTransactionId,
        sequence: Sequence,
    ) -> Result<journal::transaction::special::column::sum::ActiveModel, ServiceError>;
}
