//! Argentina identification validator.
//!
//! Validates DNI (7-8 digits) and CUIT/CUIL (11 digits) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "ARG";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/cuit.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Cuit, Validation};

/// Zero-sized marker type implementing [`Validator`] for Argentina.
pub struct IdenumbraARG;

impl Validator for IdenumbraARG {
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
        assert!(validate("0000000").is_err());
        assert!(validate("00000000000").is_err());
    }

    #[test]
    fn accepts_dni_and_cuit() {
        assert!(validate("12345678").is_ok());
        assert!(validate("12.345.678").is_ok());
        assert!(validate("20-12345678-6").is_ok());
        assert!(validate("20123456786").is_ok());
        // Remainder 1 is accepted as check digit 9 (public convention).
        assert!(validate("20000000019").is_ok());
    }

    #[test]
    fn rejects_bad_cuit() {
        assert_eq!(
            validate("20123456785"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 6,
                received: 5
            })
        );
        // Contingency prefixes (25/26) are valid and new ones are random.
        assert!(validate("25123456788").is_ok());
        assert!(validate("123456789").is_err());
    }

    #[test]
    fn honours_document_kind() {
        assert!(validate_with("12345678", "dni").is_ok());
        assert!(validate_with("12345678", "cuit").is_err());
        assert!(validate_with("20-12345678-6", "cuil").is_ok());
        assert!(validate_with("12345678", "passport").is_err());
        // The all-zero placeholder is rejected for an explicit kind too.
        assert!(validate_with("0000000", "dni").is_err());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_dni() {
        let v = validated("12345678");
        assert_eq!(v.kind, "dni");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_of_cuit() {
        let v = validated("20-12345678-6");
        assert_eq!(v.kind, "cuit");
        let cuit = match v.components {
            Some(Components::Cuit(c)) => c,
            _ => panic!("expected cuit"),
        };
        assert_eq!(cuit.prefix, 20);
        assert_eq!(cuit.dni, 12345678);
        assert_eq!(cuit.check_digit, 6);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("20123456785").is_err());
    }
}
