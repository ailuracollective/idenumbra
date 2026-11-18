//! Panama identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "PAN";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraPAN;

impl Validator for IdenumbraPAN {
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
        assert!(validate("0000000").is_err());
    }

    #[test]
    fn accepts_cedula_and_ruc_shapes() {
        assert!(validate("8-123-4567").is_ok());
        assert!(validate("PE-12-3456").is_ok());
        assert!(validate("8AV-123-4567").is_ok());
        assert!(validate("2588017-1-831938").is_ok());
        assert!(validate("81234567").is_ok());
    }

    #[test]
    fn rejects_malformed_parts() {
        assert!(validate("0-123-4567").is_err());
        assert!(validate("8-123-4567890123").is_err());
        assert!(validate("8-123").is_err());
        assert!(validate("AB-12-34").is_err());
        assert!(validate("8-12A-4567").is_err());
    }
}
