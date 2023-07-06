use crate::entity::ledger_line;

use super::Resource;

impl Resource for ledger_line::ActiveModel {
    const NAME: &'static str = "ledger_line";
}
