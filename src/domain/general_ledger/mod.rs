use crate::shared_kernel::{ArrayString128, ArrayString3};

use self::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId};

pub mod general_ledger_id;
pub mod ledger;
pub mod ledger_id;
pub mod service;

#[derive(Clone, Copy, Debug)]
pub struct GeneralLedger {
    pub id: GeneralLedgerId,
    pub name: ArrayString128,
    pub root: LedgerId,
    pub currency_code: ArrayString3,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::shared_kernel::{ArrayString128, ArrayString3};

    use super::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId, GeneralLedger};

    #[test]
    fn given_GeneralLedger_when_update_currency_then_get_currency_returns_new_currency() {
        // Arrange
        let currency_code = ArrayString3::from_str("EUR").unwrap();
        let gl = GeneralLedger {
            id: GeneralLedgerId::new(),
            name: ArrayString128::from_str("My Company").unwrap(),
            root: LedgerId::default(),
            currency_code,
        };

        // Act
        gl.update_currency_code(&ArrayString3::from_str("USD").unwrap());

        // Assert
        assert_eq!(gl.currency_code(), ArrayString3::from_str("USD").unwrap(),);
    }
}
