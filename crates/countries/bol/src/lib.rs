//! Bolivia identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "BOL";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraBOL;

impl Validator for IdenumbraBOL {
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
    fn accepts_ci_and_nit() {
        assert!(validate("1234567").is_ok());
        assert!(validate("1234567-1A").is_ok());
        assert!(validate("1234567012").is_ok());
        assert!(validate_with("1234567012", "nit").is_ok());
    }

    #[test]
    fn checks_nit_control_codes() {
        // No check digit exists: the last three digits are the SIN control
        // code (01, 02, 04, plus 03 seen in SIN lists) and one random digit.
        assert!(validate_with("123456012", "nit").is_ok()); // 01, 9 digits
        assert!(validate_with("12345670011", "nit").is_ok()); // 01, 11 digits
        assert!(validate_with("1234567029", "nit").is_ok());
        assert!(validate_with("1234567037", "nit").is_ok()); // 03
        assert!(validate_with("1234567045", "nit").is_ok());
        // The SIAT page says the final digit is 1-9; SIN supplier lists also
        // contain 0, so it is accepted (compatibility, not the published rule).
        assert!(validate_with("1234567010", "nit").is_ok());
        assert!(validate_with("123456030", "nit").is_ok()); // 03 and final 0
        assert_eq!(
            validate_with("1234567057", "nit"),
            Err(ValidationError::InvalidFormat) // undocumented code 05
        );
        assert_eq!(
            validate_with("1234567889", "nit"),
            Err(ValidationError::InvalidFormat) // no control code
        );
        assert_eq!(
            validate_with("12345678", "nit"),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: 8 // below the 9 digits of the shortest NIT in SIN lists
            })
        );
        assert_eq!(
            validate_with("123456701234", "nit"),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: 12
            })
        );
        assert!(validate("1234567890123").is_err()); // 13 digits
        assert!(validate("0000000").is_err()); // all-zero CI
        assert!(validate_with("0000000000", "nit").is_err());
        assert_eq!(validate("1234567-ABC"), Err(ValidationError::InvalidFormat));
        assert!(validate("1234").is_err());
    }
}
