//! Venezuela identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "VEN";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraVEN;

impl Validator for IdenumbraVEN {
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
        assert!(validate("000000").is_err());
        assert!(validate("V000000000").is_err());
    }

    #[test]
    fn accepts_ci_and_rif() {
        assert!(validate("12345678").is_ok());
        assert!(validate("V-12345678-1").is_ok());
        assert!(validate("V123456781").is_ok());
        assert!(validate("J123456784").is_ok());
        // Comuna RIF: the C adds the same base as J.
        assert!(validate("C123456784").is_ok());
        assert!(validate("J123456789").is_err());
    }

    #[test]
    fn rejects_bad_rif() {
        assert_eq!(
            validate("V123456782"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 2
            })
        );
        assert!(validate_with("V123456781", "rif").is_ok());
        assert!(validate_with("12345678", "rif").is_err());
        // The all-zero placeholder is rejected for an explicit kind too.
        assert!(validate_with("000000", "id_card").is_err());
    }
}
