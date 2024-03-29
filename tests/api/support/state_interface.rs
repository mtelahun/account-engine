use account_engine::{
    domain::{
        entity::ledger::ledger_id::LedgerId, external::ExternalAccount,
        subsidiary_ledger::subleder_id::SubLedgerId, LedgerAccount, ServiceError,
    },
    resource::{journal, subsidiary_ledger, LedgerType},
    shared_kernel::JournalId,
};
use rust_decimal::Decimal;

use super::CreateLedgerType;

#[async_trait::async_trait]
pub trait StateInterface {
    async fn create_account(
        &self,
        number: &str,
        name: &'static str,
        subledger_id: SubLedgerId,
    ) -> ExternalAccount;

    async fn create_ledger(&self, name: &'static str, typ: LedgerType) -> LedgerAccount;

    async fn create_ledger_leaf(&self) -> LedgerAccount;

    async fn create_subsidiary(
        &self,
        name: &'static str,
        control: CreateLedgerType,
    ) -> (
        subsidiary_ledger::ActiveModel,
        journal::ActiveModel,
        LedgerAccount,
        ExternalAccount,
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    );

    async fn create_journal(
        &self,
        code: &'static str,
        name: &'static str,
    ) -> Result<journal::ActiveModel, ServiceError>;

    async fn create_journal_xact(
        &self,
        amount: Decimal,
        account_dr_id: LedgerId,
        account_cr_id: LedgerId,
        desc: &str,
        journal_id: Option<JournalId>,
    ) -> Result<journal::transaction::general::ActiveModel, ServiceError>;

    async fn create_template_journal(
        &self,
        ledger_dr_id: LedgerId,
        ledger_cr_id: LedgerId,
    ) -> (
        journal::transaction::special::template::ActiveModel,
        Vec<journal::transaction::special::template::column::ActiveModel>,
    );
}
