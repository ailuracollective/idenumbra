//! Panama identification validator.

#![no_std]

#[cfg(test)]
use idenumbra_core::ValidationError;

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraPAN, "PAN");

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
}
