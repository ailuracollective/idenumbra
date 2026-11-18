//! Paraguay identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "PRY";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraPRY;

impl Validator for IdenumbraPRY {
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
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn accepts_ci_and_ruc() {
        assert!(validate("1234567").is_ok());
        assert!(validate("1946520-3").is_ok());
        assert!(validate("80028061-0").is_ok());
    }

    #[test]
    fn rejects_bad_ruc() {
        assert_eq!(
            validate("19465204"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 3,
                received: 4
            })
        );
        assert!(validate("800532492").is_err());
        assert!(validate("1234567890").is_err());
    }
}
