//! Venezuela identification validator.
//!
//! Validates CI (6-8 digits), RIF (letter + 8 digits + check digit) and
//! passport (6-12 alphanumerics) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered in
//! the orchestrator. Validated numbers can also be split into their parts with
//! [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "VEN";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Rif, Validation};

/// Zero-sized marker type implementing [`Validator`] for Venezuela.
pub struct IdenumbraVEN;

impl Validator for IdenumbraVEN {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
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
        assert!(validate("V000000000").is_err());
    }

    #[test]
    fn accepts_ci_and_rif() {
        assert!(validate("12345678").is_ok());
        assert!(validate("V-12345678-1").is_ok());
        assert!(validate("V123456781").is_ok());
        assert!(validate("J123456784").is_ok());
        // Comuna RIF: the C adds the same base as J.
        assert!(validate("C123456784").is_ok());
        assert!(validate("J123456789").is_err());
    }

    #[test]
    fn rejects_bad_rif() {
        assert_eq!(
            validate("V123456782"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 2
            })
        );
        assert!(validate_with("V123456781", "rif").is_ok());
        assert!(validate_with("12345678", "rif").is_err());
        // The all-zero placeholder is rejected for an explicit kind too.
        assert!(validate_with("000000", "id_card").is_err());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_ci() {
        let v = validated("12345678");
        assert_eq!(v.kind, "ci");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_of_rif() {
        let v = validated("V-12345678-1");
        assert_eq!(v.kind, "rif");
        let rif = match v.components {
            Some(Components::Rif(r)) => r,
            _ => panic!("expected rif"),
        };
        assert_eq!(rif.letter, b'V');
        assert_eq!(rif.embedded_id, 12345678);
        assert_eq!(rif.check_digit, 1);
    }

    #[test]
    fn components_of_passport() {
        let v = validated("AB123456");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("V123456782").is_err());
    }
}
