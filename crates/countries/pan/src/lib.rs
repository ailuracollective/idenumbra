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
        assert!(validate("4PI-12-345").is_ok());
        assert!(validate("13-45-678").is_ok());
        assert!(validate("2588017-1-831938").is_ok());
        assert!(validate("81234567").is_ok());
    }

    #[test]
    fn accepts_the_nt_tax_number() {
        // RUC NT (DGI): province-NT-folio-asiento.
        assert!(validate("8-NT-1-13656").is_ok());
        assert!(validate("3-NT-3-33").is_ok());
        assert!(validate_with("8-NT-1-24", "ruc").is_ok());
        // The NT tag only makes sense after a province code.
        assert!(validate("14-NT-1-24").is_err());
        assert!(validate("8-NT-12345-24").is_err());
        assert!(validate("8-NTX-1-24").is_err());
    }

    #[test]
    fn rejects_undocumented_series() {
        // EE and SB are old transitory/San Blas series with no published shape;
        // they stay rejected until the Tribunal Electoral documents them.
        assert!(validate("8SB-1-234").is_err());
        assert!(validate("EE-8-12345").is_err());
    }

    #[test]
    fn rejects_malformed_parts() {
        assert!(validate("0-123-4567").is_err());
        assert!(validate("8-123-4567890123").is_err());
        assert!(validate("8-123").is_err());
        assert!(validate("AB-12-34").is_err());
        assert!(validate("8-12A-4567").is_err());
        assert!(validate("8-NT-1-24-7").is_err());
    }
}
