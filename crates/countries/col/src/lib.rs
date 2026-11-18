//! Colombia identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "COL";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraCOL;

impl Validator for IdenumbraCOL {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = ();
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
        assert!(validate_with("0000000000", "nit").is_err());
    }

    #[test]
    fn accepts_citizen_id_and_passport() {
        assert!(validate("12345678").is_ok());
        assert!(validate("1234567890").is_ok()); // 10-digit cédulas exist
        assert!(validate("AB123456").is_ok());
    }

    #[test]
    fn validates_nit_check_digit() {
        // Weights run right to left: 3, 7, 13, ... 41.
        assert!(validate_with("9006578000", "nit").is_ok());
        assert!(validate_with("9876543217", "nit").is_ok());
        assert_eq!(
            validate_with("9006578001", "nit"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 0,
                received: 1
            })
        );
    }

    #[test]
    fn accepts_short_and_long_natural_nits() {
        // A natural person's NIT is their cédula plus the check digit, so it
        // can be shorter or longer than the 9-digit company body.
        assert!(validate_with("123456788", "nit").is_ok());
        assert!(validate_with("10987654329", "nit").is_ok());
        assert!(validate_with("10987654328", "nit").is_err());
        // The longest body the DIAN describes: an 11-digit civil registry
        // number, 12 digits with the check digit.
        assert!(validate_with("109876543214", "nit").is_ok());
        assert_eq!(
            validate_with("109876543215", "nit"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 4,
                received: 5
            })
        );
    }

    #[test]
    fn rejects_bad_lengths() {
        assert!(validate("12345").is_err());
        assert!(validate("12345678901").is_err());
        assert!(validate_with("1234567890123", "nit").is_err()); // 13 digits
    }
}
