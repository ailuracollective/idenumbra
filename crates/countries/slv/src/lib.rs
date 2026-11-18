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
    fn accepts_both_nit_tables_for_legacy_correlatives() {
        // Correlative 001: the legacy table gives 0 and the current one 7.
        // The 014-100 band is unobserved in public samples, so either passes.
        assert!(validate("0614-010120-001-0").is_ok());
        assert!(validate("0614-010120-001-7").is_ok());
    }

    #[test]
    fn uses_only_the_current_table_above_the_cut_off() {
        // Correlative 200: the current table gives 1; the legacy 5 is not
        // accepted, because every public sample with a correlative of 101 or
        // more follows the current table.
        assert!(validate("0614-010120-200-1").is_ok());
        assert_eq!(
            validate("0614-010120-200-5"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 5
            })
        );
    }

    #[test]
    fn the_cut_off_is_inclusive_at_100() {
        // Correlative 100 is the last legacy one: the legacy sum is 166
        // (mod 11 = 1 -> 1) and the current one 94 (mod 11 = 6 -> 5); both are
        // accepted.
        assert!(validate("0614-010120-100-1").is_ok());
        assert!(validate("0614-010120-100-5").is_ok());
        // Correlative 102 (101 gives the same digit under both tables) already
        // uses only the current table: legacy 170 (mod 11 = 5 -> 5), current 98
        // (mod 11 = 10 -> 1).
        assert!(validate("0614-010120-102-1").is_ok());
        assert_eq!(
            validate("0614-010120-102-5"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 5
            })
        );
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
