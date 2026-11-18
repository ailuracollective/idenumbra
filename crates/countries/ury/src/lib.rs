//! Uruguay identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "URY";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rut.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraURY;

impl Validator for IdenumbraURY {
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
    fn accepts_ci_and_rut() {
        assert!(validate("1234567-2").is_ok());
        assert!(validate("12345672").is_ok());
        assert!(validate("21-999983-001-9").is_ok());
        assert!(validate("219999830019").is_ok());
    }

    #[test]
    fn rejects_bad_ci_and_rut() {
        assert_eq!(
            validate("12345673"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 2,
                received: 3
            })
        );
        assert!(validate("219999830018").is_err());
        assert_eq!(
            validate("239999830019"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        assert!(validate("219999830029").is_err()); // positions 9-11 are 001
    }
}
