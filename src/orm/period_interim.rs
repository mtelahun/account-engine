use crate::entity::interim_accounting_period;

use super::Resource;

impl Resource for interim_accounting_period::ActiveModel {
    const NAME: &'static str = "interim_accounting_period";
}
