//! Guatemala identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "GTM";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/cui.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraGTM;

impl Validator for IdenumbraGTM {
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
        assert!(validate("0000000000000").is_err());
    }

    #[test]
    fn accepts_cui_and_nit() {
        assert!(validate("1234567890101").is_ok());
        assert!(validate("12345679").is_ok());
        assert!(validate("1023456K").is_ok());
    }

    #[test]
    fn rejects_bad_cui() {
        assert_eq!(
            validate("1234567880101"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 9,
                received: 8
            })
        );
        assert_eq!(
            validate("1234567892301"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        // Department 01 only has 17 municipalities.
        assert!(validate("1234567890118").is_err());
        // Sipacate (0514, Decreto 4-2015) and Petatán (1333, Decreto 6-2015).
        assert!(validate("1234567890514").is_ok());
        assert!(validate("1234567891333").is_ok());
        // Sipacate (Escuintla 14) and Petatán (Huehuetenango 33) exist.
        assert!(validate("1234567890514").is_ok());
        assert!(validate("1234567891333").is_ok());
        assert!(validate("1234567890515").is_err());
        assert!(validate("1234567891334").is_err());
    }

    #[test]
    fn rejects_bad_nit() {
        assert!(validate("12345678").is_err());
        assert!(validate("1234567K").is_err());
    }
}
