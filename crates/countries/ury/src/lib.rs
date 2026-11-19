//! Uruguay identification validator.
//!
//! Validates CI (cédula de identidad), RUT and passport numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rut.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Ci, Components, Rut, Validation};

/// Zero-sized marker type implementing [`Validator`] for Uruguay.
pub struct IdenumbraURY;

impl Validator for IdenumbraURY {
    const COUNTRY: &'static str = "URY";
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
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn accepts_ci_and_rut() {
        assert!(validate("1234567-2").is_ok());
        assert!(validate("12345672").is_ok());
        assert!(validate("21-999983-001-9").is_ok());
        assert!(validate("219999830019").is_ok());
        // 22 is the only prefix the DGI assigns since October 2024.
        assert!(validate("22-999983-001-6").is_ok());
    }

    #[test]
    fn pads_six_digit_ci_to_seven() {
        // Body 123456 pads to 0123456, sum 109, digit 1.
        assert!(validate("1234561").is_ok());
        assert!(validate("01234561").is_ok());
        assert_eq!(
            validate("1234562"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 2
            })
        );
    }

    #[test]
    fn rejects_unrepresentable_rut_check_digit() {
        // These eleven digits leave remainder 1, so the digit would be 10 and
        // no RUT with them can exist.
        assert_eq!(
            validate("219999740019"),
            Err(ValidationError::UnrepresentableCheckDigit)
        );
    }

    #[test]
    fn rejects_bad_ci_and_rut() {
        assert_eq!(
            validate("12345673"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 2,
                received: 3
            })
        );
        assert!(validate("219999830018").is_err());
        assert_eq!(
            validate("239999830019"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        assert!(validate("219999830029").is_err()); // positions 9-11 are 001
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_ci() {
        let v = validated("12345672");
        assert_eq!(v.kind, "ci");
        let c = match v.components {
            Some(Components::Ci(c)) => c,
            _ => panic!("expected ci"),
        };
        assert_eq!(c.check_digit, 2);
    }

    #[test]
    fn components_of_rut() {
        let v = validated("219999830019");
        assert_eq!(v.kind, "rut");
        let r = match v.components {
            Some(Components::Rut(r)) => r,
            _ => panic!("expected rut"),
        };
        assert_eq!(r.operating_unit, 21);
        assert_eq!(r.sequential, 999983);
        assert_eq!(r.check_digit, 9);
    }

    #[test]
    fn components_of_passport() {
        let v = validated("AB123456");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("12345673").is_err());
    }
}
