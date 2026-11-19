//! Guatemala identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/components.rs");
    include!("validators/validate.rs");
    include!("validators/cui.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{components, validate, validate_with, Components, Cui, Nit, Validation};

/// Zero-sized marker type implementing [`Validator`] for Guatemala.
pub struct IdenumbraGTM;

impl Validator for IdenumbraGTM {
    const COUNTRY: &'static str = "GTM";
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
    fn document_kinds() -> &'static [&'static str] {
        &["cui", "nit", "passport"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fixed catalogue boundaries remain here; generated properties cover the
    // general region rules without guaranteeing every documented boundary.
    #[test]
    fn cui_catalogue_boundaries() {
        for value in [
            "1234567890101", // Guatemala, 01-01
            "1234567892217", // Jutiapa, 22-17 (last department and code)
            "1234567890514", // Sipacate, Escuintla, 05-14
            "1234567891333", // Petatán, Huehuetenango, 13-33
        ] {
            assert!(validate_with(value, "cui").is_ok(), "{value}");
        }

        assert_eq!(
            validate("1234567892301"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        assert_eq!(
            validate("1234567890001"),
            Err(ValidationError::InvalidRegion { region: 0 })
        );
        for value in [
            "1234567892218",
            "1234567890118",
            "1234567890515",
            "1234567891334",
            "1234567890500",
        ] {
            assert_eq!(
                validate_with(value, "cui"),
                Err(ValidationError::InvalidFormat),
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_cui_whose_check_digit_would_be_10() {
        // 10000007 * 2..9 sums to a remainder of 10, which is not a digit, so
        // SAT's rule (§5.1.4.4, sum * 10 mod 11) has nothing to print.
        assert_eq!(
            validate("1000000700101"),
            Err(ValidationError::UnrepresentableCheckDigit)
        );
        // The same body has no valid 9-digit CUI-prefix NIT either.
        assert!(validate("100000070").is_err());
    }
}
