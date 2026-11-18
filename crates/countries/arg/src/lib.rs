//! Argentina identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "ARG";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/cuit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraARG;

impl Validator for IdenumbraARG {
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
        assert!(validate("0000000").is_err());
        assert!(validate("00000000000").is_err());
    }

    #[test]
    fn accepts_dni_and_cuit() {
        assert!(validate("12345678").is_ok());
        assert!(validate("12.345.678").is_ok());
        assert!(validate("20-12345678-6").is_ok());
        assert!(validate("20123456786").is_ok());
        // Remainder 1 is accepted as check digit 9 (public convention).
        assert!(validate("20000000019").is_ok());
    }

    #[test]
    fn rejects_bad_cuit() {
        assert_eq!(
            validate("20123456785"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 6,
                received: 5
            })
        );
        // Contingency prefixes (25/26) are valid and new ones are random.
        assert!(validate("25123456788").is_ok());
        assert!(validate("123456789").is_err());
    }

    #[test]
    fn honours_document_kind() {
        assert!(validate_with("12345678", "dni").is_ok());
        assert!(validate_with("12345678", "cuit").is_err());
        assert!(validate_with("20-12345678-6", "cuil").is_ok());
        assert!(validate_with("12345678", "passport").is_err());
        // The all-zero placeholder is rejected for an explicit kind too.
        assert!(validate_with("0000000", "dni").is_err());
    }
}
