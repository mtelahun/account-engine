use crate::{
    domain::{
        AccountId, ArrayString24, LedgerId, Sequence, SpecialJournalTemplateId, TemplateColumnId,
    },
    resource::journal::transaction::JournalTransactionColumnType,
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
