//! Nicaragua identification validator.
//!
//! Validates identity card (cédula) and passport numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "NIC";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Cedula, Components, Validation};

/// Zero-sized marker type implementing [`Validator`] for Nicaragua.
pub struct IdenumbraNIC;

impl Validator for IdenumbraNIC {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
    fn document_kinds() -> &'static [&'static str] {
        &["id_card", "passport"]
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

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_cedula() {
        let v = validated("0010101900001N");
        assert_eq!(v.kind, "id_card");
        let card = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => panic!("expected cedula"),
        };
        assert_eq!(card.municipality, 1);
        assert_eq!(card.birth_date, *b"010190");
        assert_eq!(card.serial, 1);
        assert_eq!(card.check_letter, b'N');
    }

    #[test]
    fn components_of_passport() {
        let v = validated("A1234567");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("0010101900001A").is_err());
    }
}
