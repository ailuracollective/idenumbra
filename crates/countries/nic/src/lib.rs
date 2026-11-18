//! Nicaragua identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "NIC";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraNIC;

impl Validator for IdenumbraNIC {
    const COUNTRY: &'static str = COUNTRY;
    fn validate(value: &str) -> Result<(), ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
        crate::validate_with(value, kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_identity_card() {
        assert!(validate("0010101900001N").is_ok());
        assert!(validate("001-010190-0001N").is_ok());
    }

    #[test]
    fn rejects_bad_letter_and_date() {
        assert_eq!(
            validate("0010101900001A"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 12,
                received: 0
            })
        );
        assert!(validate("0013201900001N").is_err()); // month 32
        assert!(validate("001010190000").is_err());
        assert!(validate("001010190001").is_err()); // missing letter
    }

    #[test]
    fn passport_kind() {
        assert!(validate_with("A1234567", "passport").is_ok());
    }
}
