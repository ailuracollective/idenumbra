//! Chile identification validator (RUT/RUN).

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "CHL";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rut.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraCHL;

impl Validator for IdenumbraCHL {
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
        assert!(validate("00000000-0").is_err());
    }

    #[test]
    fn accepts_rut() {
        assert!(validate("12.345.678-5").is_ok());
        assert!(validate("123456785").is_ok());
        // Check digit 10 is written as K.
        assert!(validate("12.345.670-K").is_ok());
        assert!(validate("12345670k").is_ok());
    }

    #[test]
    fn rejects_bad_rut() {
        assert!(validate("12345678-4").is_err());
        assert!(validate("12345678-K").is_err());
        assert!(validate("1234567890").is_err());
        assert!(validate("76086A28-5").is_err());
    }
}
