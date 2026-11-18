//! El Salvador identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "SLV";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/dui.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraSLV;

impl Validator for IdenumbraSLV {
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
        assert!(validate_with("00000000000000", "nit").is_err());
    }

    #[test]
    fn accepts_dui_and_nit() {
        assert!(validate("12345678-4").is_ok());
        assert!(validate("123456784").is_ok());
        assert!(validate("0614-050707-104-8").is_ok());
        assert!(validate("1234-567890-123-0").is_ok());
    }

    #[test]
    fn rejects_bad_check_digits() {
        assert_eq!(
            validate("123456785"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 4,
                received: 5
            })
        );
        assert!(validate("0614-050707-104-0").is_err());
        assert!(validate("00000000-0").is_err());
        assert!(validate("2234-567890-123-0").is_err()); // bad first digit
    }
}
