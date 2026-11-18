//! Ecuador identification validator.
//!
//! Validates ID card (10 digits), RUC (13 digits) and passport (alphanumeric)
//! numbers. Implements [`idenumbra_core::Validator`], so it can be used on its
//! own or registered in the orchestrator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "ECU";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/id_card.rs");
    include!("validators/passport.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Zero-sized marker type implementing [`Validator`] for Ecuador.
pub struct IdenumbraECU;

impl Validator for IdenumbraECU {
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
    fn accepts_id_card_and_rucs() {
        assert!(validate("1712345675").is_ok());
        assert!(validate("1712345675001").is_ok()); // natural person
        assert!(validate("1790012344001").is_ok()); // private company
        assert!(validate("1760001200001").is_ok()); // public company

        // The SRI stopped guaranteeing the modulo 11 digit for private RUCs
        // whose sequential passes six digits, so that digit is not checked.
        assert!(validate("1790012345001").is_ok());
        assert!(validate("5090012340001").is_ok()); // legacy code 50
    }

    #[test]
    fn rejects_bad_check_digits() {
        assert_eq!(
            validate("1712345676"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 5,
                received: 6
            })
        );
        assert!(validate("1760001210001").is_err()); // public company
    }

    #[test]
    fn rejects_bad_structure() {
        assert_eq!(
            validate("2512345675"),
            Err(ValidationError::InvalidRegion { region: 25 })
        );
        assert_eq!(
            validate("5012345675"),
            Err(ValidationError::InvalidRegion { region: 50 })
        );
        // RUC always ends in 001 (art. 3 of the RUC reglamento).
        assert!(validate("1712345675002").is_err());
        assert!(validate("1790012344002").is_err());
        assert!(validate("1760001200002").is_err());
        assert!(validate("1770012340001").is_err()); // third digit 7
        assert!(validate("17123458").is_err());
    }

    #[test]
    fn accepts_hyphen_before_last_digit() {
        assert!(validate("171234567-5").is_ok());
        assert!(validate("171234567500-1").is_ok());
        assert!(validate_with("171234567-5", "id_card").is_ok());
        assert!(validate_with("171234567500-1", "ruc").is_ok());
        // Only that position: anywhere else the hyphen makes it a bad passport.
        assert!(validate("171234-5675").is_err());
        assert!(validate("17-12345675").is_err());
        assert!(validate_with("1712345-675", "id_card").is_err());
    }

    #[test]
    fn kinds() {
        assert!(validate_with("1712345675", "id_card").is_ok());
        assert!(validate_with("1712345675001", "ruc").is_ok());
        assert!(validate_with("1712345675", "ruc").is_err());
        assert!(validate_with("A123456", "passport").is_ok());
    }
}
