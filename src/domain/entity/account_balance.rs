use rust_decimal::Decimal;

use super::xact_type::XactType;

pub struct AccountBalance {
    pub amount: Decimal,
    pub xact_type: XactType,
}
