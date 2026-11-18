//! Nicaragua identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "NIC";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraNIC;

impl Validator for IdenumbraNIC {
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
    fn accepts_identity_card() {
        assert!(validate("0010101900001N").is_ok());
        assert!(validate("001-010190-0001N").is_ok());
        assert!(validate("001 010190 0001n").is_ok());
        // Municipality codes are not validated, including 888 (born abroad,
        // single third-party source).
        assert!(validate("8880101900001P").is_ok());
    }

    #[test]
    fn rejects_bad_letter_and_date() {
        assert_eq!(
            validate("0010101900001A"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 12,
                received: 0
            })
        );
        // I, O and Z are not in the 23-letter alphabet, so they have no index.
        assert_eq!(
            validate("0010101900001I"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 12,
                received: 23
            })
        );
        assert!(validate("0013201900001N").is_err()); // month 32
        assert!(validate("0012902300001N").is_err()); // no 29 February in 2023
        assert!(validate("0012902400001C").is_ok()); // 29 February 2024
        assert_eq!(
            validate("001010190000"),
            Err(ValidationError::InvalidLength {
                expected: 14,
                found: 12
            })
        );
        assert_eq!(
            validate("0010101900001"),
            Err(ValidationError::InvalidLength {
                expected: 14,
                found: 13
            })
        );
        assert_eq!(
            validate("00101019000011"),
            Err(ValidationError::InvalidFormat) // digit instead of letter
        );
        assert_eq!(
            validate("0A10101900001N"),
            Err(ValidationError::NonNumeric { index: 1 })
        );
    }

    #[test]
    fn passport_inferred_and_explicit() {
        assert!(validate("A1234567").is_ok());
        assert!(validate_with("A1234567", "passport").is_ok());
        // A digit-only run shorter than a cédula is a malformed cédula, not a
        // passport.
        assert_eq!(
            validate("12345678"),
            Err(ValidationError::InvalidLength {
                expected: 14,
                found: 8
            })
        );
        // The kind is strict: neither document passes as the other.
        assert_eq!(
            validate_with("A1234567", "id_card"),
            Err(ValidationError::InvalidLength {
                expected: 14,
                found: 8
            })
        );
        assert_eq!(
            validate_with("0010101900001N", "passport"),
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: 14
            })
        );
    }

    #[test]
    fn unknown_kind_is_rejected() {
        assert_eq!(
            validate_with("0010101900001N", "nit"),
            Err(ValidationError::InvalidFormat)
        );
    }
}
