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
    fn century_digit_selects_the_century() {
        // The 7th digit is a century marker: 9 = 18xx, 0-5 = 19xx, 6-8 = 20xx.
        assert!(validate("01010190000").is_ok()); // 1801-01-01
        assert!(validate("01010100000").is_ok()); // 1901-01-01
        assert!(validate("99123150000").is_ok()); // 1999-12-31, 19xx with 5
        assert!(validate("01010160000").is_ok()); // 2001-01-01
        assert!(validate("99123180000").is_ok()); // 2099-12-31, 20xx with 8

        // 1800 was not a leap year.
        assert!(validate("00022990000").is_err());
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
