use crate::entity::ledger_transaction;

use super::Resource;

impl Resource for ledger_transaction::ActiveModel {
    const NAME: &'static str = "ledger_transaction";
}
