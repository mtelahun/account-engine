use std::str::FromStr;

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

impl GeneralLedger {
    pub fn update_currency_code(&mut self, code: &str) -> ArrayString3 {
        let previous_code = self.currency_code;
        self.currency_code = ArrayString3::from_str(code).unwrap();

        previous_code
    }

    pub fn currency_code(&self) -> ArrayString3 {
        self.currency_code
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::shared_kernel::{ArrayString128, ArrayString3};

    use super::{general_ledger_id::GeneralLedgerId, ledger_id::LedgerId, GeneralLedger};

    #[test]
    #[allow(non_snake_case)]
    fn given_GeneralLedger_when_update_currency_then_get_currency_returns_new_currency() {
        // Arrange
        let currency_code = ArrayString3::from_str("EUR").unwrap();
        let mut gl = GeneralLedger {
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

    #[test]
    #[allow(non_snake_case)]
    fn given_GeneralLedger_when_update_name_then_get_name_returns_new_name() {
        // Arrange
        let name = ArrayString128::from_str("My New Company Name").unwrap();
        let mut gl = GeneralLedger {
            id: GeneralLedgerId::new(),
            name: ArrayString128::from_str("My Company").unwrap(),
            root: LedgerId::default(),
            currency_code: ArrayString3::from_str("EUR").unwrap(),
        };

        // Act
        gl.update_name(&name);

        // Assert
        assert_eq!(gl.name(), name,);
    }
}
