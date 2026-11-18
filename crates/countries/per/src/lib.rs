//! Peru identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "PER";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraPER;

impl Validator for IdenumbraPER {
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
    fn rejects_all_zero_placeholders() {
        // Placeholder values are never issued.
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn accepts_dni_and_ruc() {
        assert!(validate("12345678").is_ok());
        assert!(validate("20123456786").is_ok());
        assert!(validate("AB123456").is_ok());
    }

    #[test]
    fn rejects_bad_ruc() {
        assert_eq!(
            validate("20123456787"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 6,
                received: 7
            })
        );
        assert!(validate("30123456786").is_err());
        assert!(validate("2012345678").is_err());
    }
}
