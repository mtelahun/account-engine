use crate::entity::ledger_xact_type;

use super::Resource;

impl Resource for ledger_xact_type::ActiveModel {
    const NAME: &'static str = "ledger_transaction_type";
}
