//! Cuba identification validator (identity card).

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "CUB";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/date.rs");
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraCUB;

impl Validator for IdenumbraCUB {
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
    fn accepts_identity_card() {
        assert!(validate("90010112345").is_ok());
        // 1904 was a leap year; the century code selects the century.
        assert!(validate("04022912345").is_ok());
    }

    #[test]
    fn rejects_impossible_dates() {
        assert!(validate("90130112345").is_err());
        assert!(validate("90023012345").is_err());
        // 1900 was not a leap year.
        assert!(validate("00022912345").is_err());
    }

    #[test]
    fn sex_digit_constrains_nothing() {
        // 10th digit: even = male, odd = female. Both are valid digits, so the
        // position cannot reject anything without the person's registered sex.
        assert!(validate("90010112345").is_ok());
        assert!(validate("90010112355").is_ok());
        // 21st-century century code (8 = 20xx).
        assert!(validate("20010180110").is_ok());
    }
}
