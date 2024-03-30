use crate::{
    domain::entity::{
        external_account::account_id::AccountId,
        journal_transaction_column::column_type::JournalTransactionColumnType,
        ledger::ledger_id::LedgerId,
        special_journal_template::special_journal_template_id::SpecialJournalTemplateId,
        special_journal_template_column::template_column_id::TemplateColumnId,
    },
    shared_kernel::{ArrayString24, Sequence},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Model {
    pub template_id: SpecialJournalTemplateId,
    pub sequence: Sequence,
    pub name: ArrayString24,
    pub column_type: JournalTransactionColumnType,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
    pub dr_account_id: Option<AccountId>,
    pub cr_account_id: Option<AccountId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveModel {
    pub id: TemplateColumnId,
    pub template_id: SpecialJournalTemplateId,
    pub sequence: Sequence,
    pub name: ArrayString24,
    pub column_type: JournalTransactionColumnType,
    pub cr_ledger_id: Option<LedgerId>,
    pub dr_ledger_id: Option<LedgerId>,
    pub dr_account_id: Option<AccountId>,
    pub cr_account_id: Option<AccountId>,
}

impl From<&Model> for ActiveModel {
    fn from(value: &Model) -> Self {
        Self {
            id: TemplateColumnId::new(),
            template_id: value.template_id,
            sequence: value.sequence,
            name: value.name,
            column_type: value.column_type,
            cr_ledger_id: value.cr_ledger_id,
            dr_ledger_id: value.dr_ledger_id,
            dr_account_id: value.dr_account_id,
            cr_account_id: value.cr_account_id,
        }
    }
}
