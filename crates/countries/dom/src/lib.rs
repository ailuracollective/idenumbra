//! Dominican Republic identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "DOM";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraDOM;

impl Validator for IdenumbraDOM {
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
        assert!(validate("00000000000").is_err());
    }

    #[test]
    fn accepts_formatted_id_card() {
        assert!(validate("12345678903").is_ok());
        assert!(validate("123-4567890-3").is_ok());
    }

    #[test]
    fn rejects_bad_luhn_digit() {
        assert_eq!(
            validate("12345678904"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 3,
                received: 4
            })
        );
        assert!(validate("1234567890").is_err());
    }
}
