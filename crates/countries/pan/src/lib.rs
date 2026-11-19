//! Panama identification validator.
//!
//! Validates cédula (hyphenated or plain) and RUC (including the NT tax number)
//! and passport numbers. Implements [`idenumbra_core::Validator`], so it can be
//! used on its own or registered in the orchestrator. Validated numbers can also
//! be split into their parts with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Cedula, Components, Validation};

/// Alias kept for callers that prefer the country-prefixed name.
pub type PanCedula = Cedula;

/// Zero-sized marker type implementing [`Validator`] for Panama.
pub struct IdenumbraPAN;

impl Validator for IdenumbraPAN {
    const COUNTRY: &'static str = "PAN";
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
        // EE and SB are old transitory/San Blas series: Decreto 53/1958 art. 9
        // lists only province, PE, N and E, and no later rule defines them.
        assert!(validate("8SB-1-234").is_err());
        assert!(validate("3SB-1-234").is_err());
        assert!(validate("EE-8-12345").is_err());
        // E-NT is not a shape either: the DGI form has one "N° de Cédula o N.T"
        // field, and an NT is written province-NT-folio-asiento.
        assert!(validate("E-NT-8-123").is_err());
    }

    #[test]
    fn kinds_are_strict() {
        // A passport is neither a cédula nor a RUC.
        assert_eq!(
            validate_with("AB123456", "id_card"),
            Err(ValidationError::InvalidFormat)
        );
        assert_eq!(
            validate_with("AB123456", "ruc"),
            Err(ValidationError::InvalidFormat)
        );
        // The four-part NT is a tax number, not a cédula.
        assert_eq!(
            validate_with("8-NT-1-24", "id_card"),
            Err(ValidationError::InvalidFormat)
        );
        assert!(validate_with("8-NT-1-24", "ruc").is_ok());
        // A natural person's RUC is their cédula.
        assert!(validate_with("8-123-4567", "ruc").is_ok());
        assert!(validate_with("8-123-4567", "id_card").is_ok());
        assert!(validate_with("81234567", "ruc").is_ok());
        assert!(validate_with("81234567", "id_card").is_ok());
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

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_cedula() {
        let v = validated("8-123-4567");
        assert_eq!(v.kind, "id_card");
        let c = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => panic!("expected cedula"),
        };
        assert_eq!(c.province, 0);
        assert_eq!(c.marker_len, 0);
        assert_eq!(c.folio, 123);
        assert_eq!(c.asiento, 4567);
        assert!(!c.is_nt);
    }

    #[test]
    fn components_of_marker() {
        let v = validated("PE-12-3456");
        let c = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => panic!("expected cedula"),
        };
        assert_eq!(c.province, 0);
        assert_eq!(c.marker_len, 2);
        assert_eq!(&c.marker[..c.marker_len as usize], b"PE");
        assert_eq!(c.folio, 12);
        assert_eq!(c.asiento, 3456);
    }

    #[test]
    fn components_of_province_marker() {
        let v = validated("8AV-123-4567");
        let c = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => panic!("expected cedula"),
        };
        assert_eq!(c.province, 8);
        assert_eq!(c.marker_len, 2);
        assert_eq!(&c.marker[..c.marker_len as usize], b"AV");
    }

    #[test]
    fn components_of_nt() {
        let v = validated("8-NT-1-13656");
        assert_eq!(v.kind, "ruc");
        let c = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => panic!("expected cedula"),
        };
        assert!(c.is_nt);
        assert_eq!(c.province, 8);
        assert_eq!(c.folio, 1);
        assert_eq!(c.asiento, 13656);
    }

    #[test]
    fn components_of_plain() {
        let v = validated("81234567");
        assert_eq!(v.kind, "id_card");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_of_passport() {
        let v = validated("AB123456");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("8-123-4567890123").is_err());
    }
}
