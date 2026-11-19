//! Honduras identification validator.
//!
//! Validates DNI (13 digits), RTN (14 digits) and passport numbers.
//! Implements [`idenumbra_core::Validator`], so it can be used on its own or
//! registered in the orchestrator. Validated numbers can also be split into
//! their parts with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "HND";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rtn.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Rtn, Validation};

/// Zero-sized marker type implementing [`Validator`] for Honduras.
pub struct IdenumbraHND;

impl Validator for IdenumbraHND {
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
    fn accepts_dni_and_rtn() {
        assert!(validate("1234567890123").is_ok());
        assert!(validate("12345678901230").is_ok());
        assert!(validate("1234-5678-901230").is_ok());
        assert!(validate_with("1234567890123", "dni").is_ok());
        assert!(validate_with("12345678901230", "rtn").is_ok());
    }

    #[test]
    fn checks_structure_only() {
        // The SAR does not publish the RTN verifier algorithm: the last digit
        // is not checked.
        assert!(validate("12345678901231").is_ok());
        assert!(validate("12345678901239").is_ok());
        assert!(validate("123456789012").is_err());
        assert!(validate("0000000000000").is_err());
        assert!(validate("00000000000000").is_err());
        assert!(validate_with("1234567890123", "rtn").is_err());
        assert!(validate_with("12345678901230", "dni").is_err());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_dni() {
        let v = validated("1234567890123");
        assert_eq!(v.kind, "dni");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_of_rtn() {
        let v = validated("12345678901230");
        assert_eq!(v.kind, "rtn");
        let rtn = match v.components {
            Some(Components::Rtn(r)) => r,
            _ => panic!("expected rtn"),
        };
        assert_eq!(rtn.embedded_dni, 1234567890123);
        assert_eq!(rtn.trailing, 0);
    }

    #[test]
    fn components_of_passport() {
        let v = validated("AB1234");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_require_valid_number() {
        assert!(validate("0000000000000").is_err());
        let v = validated("AB1234");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }
}
