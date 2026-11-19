//! Peru identification validator.
//!
//! Validates DNI (8 digits), RUC (11 digits) and passport/carné de
//! extranjería (6-12 alphanumerics) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "PER";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Ruc, Validation};

/// Zero-sized marker type implementing [`Validator`] for Peru.
pub struct IdenumbraPER;

impl Validator for IdenumbraPER {
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
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn accepts_dni_and_ruc() {
        assert!(validate("12345678").is_ok());
        assert!(validate("20123456786").is_ok());
        assert!(validate("15123456782").is_ok()); // foreign natural person
        assert!(validate("17123456785").is_ok()); // prefix 17, undocumented
        assert!(validate("AB123456").is_ok());
    }

    #[test]
    fn rejects_bad_ruc() {
        assert_eq!(
            validate("20123456787"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 6,
                received: 7
            })
        );
        assert!(validate("30123456786").is_err());
        assert!(validate("2012345678").is_err());
    }
}
