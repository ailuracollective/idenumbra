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
        assert!(validate("123456789").is_ok());
        assert!(validate_with("123456789", "nit").is_ok());
    }

    #[test]
    fn checks_structure_only() {
        // The NIT has no public check digit: any 7-12 digit value with the
        // right shape is accepted.
        assert!(validate_with("123456788", "nit").is_ok());
        assert!(validate("1234567890123").is_err()); // 13 digits
        assert!(validate("0000000").is_err()); // all-zero CI
        assert!(validate_with("000000000", "nit").is_err());
        assert_eq!(validate("1234567-ABC"), Err(ValidationError::InvalidFormat));
        assert!(validate("1234").is_err());
    }
}
