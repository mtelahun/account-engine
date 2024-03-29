use crate::resource::{
    accounting_period, external, general_ledger, journal, ledger, ledger_xact_type,
    subsidiary_ledger,
};

pub trait Resource {
    const NAME: &'static str;
}

impl Resource for general_ledger::ActiveModel {
    const NAME: &'static str = "general_ledger";
}

impl Resource for accounting_period::interim_period::ActiveModel {
    const NAME: &'static str = "interim_accounting_period";
}

impl Resource for journal::transaction::general::line::ActiveModel {
    const NAME: &'static str = "journal_transaction_general";
}

impl Resource for journal::transaction::column::ledger_drcr::ActiveModel {
    const NAME: &'static str = "journal_transaction_column_ledger_drcr";
}

impl Resource for journal::transaction::column::text::ActiveModel {
    const NAME: &'static str = "journal_transaction_column_text";
}

impl Resource for journal::transaction::column::account_dr::ActiveModel {
    const NAME: &'static str = "journal_transaction_column_account_dr";
}

impl Resource for journal::transaction::column::account_cr::ActiveModel {
    const NAME: &'static str = "journal_transaction_column_account_cr";
}

impl Resource for journal::transaction::special::column::ActiveModel {
    const NAME: &'static str = "journal_transaction_special_column";
}

impl Resource for journal::transaction::special::summary::ActiveModel {
    const NAME: &'static str = "journal_transaction_special_totals";
}

impl Resource for journal::transaction::special::column::sum::ActiveModel {
    const NAME: &'static str = "journal_transaction_special_column_total";
}

impl Resource for journal::transaction::special::template::ActiveModel {
    const NAME: &'static str = "journal_transaction_special_template";
}

impl Resource for journal::transaction::special::template::column::ActiveModel {
    const NAME: &'static str = "journal_transaction_special_template_column";
}

impl Resource for journal::transaction::ActiveModel {
    const NAME: &'static str = "journal_transaction";
}

impl Resource for journal::transaction::special::ActiveModel {
    const NAME: &'static str = "journal_transaction_special";
}

/// The journal_transaction::ActiveModel is only ever used to communicate with
/// the caller and doesn't have any datastore models associated with it.
impl Resource for journal::transaction::general::ActiveModel {
    const NAME: &'static str = "";
}

impl Resource for journal::ActiveModel {
    const NAME: &'static str = "journal";
}

impl Resource for ledger::ActiveModel {
    const NAME: &'static str = "ledger";
}

impl Resource for ledger::derived::ActiveModel {
    const NAME: &'static str = "ledger_derived";
}

impl Resource for ledger::intermediate::ActiveModel {
    const NAME: &'static str = "ledger_intermediate";
}

impl Resource for ledger::leaf::ActiveModel {
    const NAME: &'static str = "ledger_leaf";
}

impl Resource for ledger::transaction::ActiveModel {
    const NAME: &'static str = "ledger_transaction";
}

impl Resource for ledger::transaction::ledger::ActiveModel {
    const NAME: &'static str = "ledger_transaction_ledger";
}

impl Resource for ledger::transaction::account::ActiveModel {
    const NAME: &'static str = "ledger_transaction_account";
}

impl Resource for ledger_xact_type::ActiveModel {
    const NAME: &'static str = "ledger_transaction_type";
}

impl Resource for accounting_period::ActiveModel {
    const NAME: &'static str = "accounting_period";
}

impl Resource for external::account::ActiveModel {
    const NAME: &'static str = "external_account";
}

impl Resource for external::account::transaction::ActiveModel {
    const NAME: &'static str = "external_account_transaction";
}

impl Resource for external::entity::ActiveModel {
    const NAME: &'static str = "external_entity";
}

impl Resource for external::entity_type::ActiveModel {
    const NAME: &'static str = "entity_type";
}

impl Resource for external::transaction_type::ActiveModel {
    const NAME: &'static str = "transaction_type_external";
}

impl Resource for subsidiary_ledger::ActiveModel {
    const NAME: &'static str = "subsidiary_ledger";
}