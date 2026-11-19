//! Costa Rica identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{
    components, validate, validate_with, CedulaFisica, CedulaJuridica, Components, Dimex,
    Validation,
};

/// Zero-sized marker type implementing [`Validator`] for Costa Rica.
pub struct IdenumbraCRI;

impl Validator for IdenumbraCRI {
    const COUNTRY: &'static str = "CRI";
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
        // Placeholder values are never issued: every format forbids a leading
        // zero.
        assert!(validate("000000000").is_err());
        assert!(validate("00000000000").is_err());
        assert!(validate("0000000000").is_err());
        assert!(validate_with("0000000000", "nite").is_err());
    }

    #[test]
    fn accepts_id_card_dimex_and_juridical() {
        assert!(validate("123456789").is_ok());
        assert!(validate("12345678901").is_ok());
        assert!(validate("123456789012").is_ok());
        // Cédula jurídica: 1 type digit, 3 class digits, 6 consecutive.
        assert!(validate("3101123456").is_ok());
        // Alphanumeric consecutive, announced by the Registro Nacional for the
        // end of 2026 (example 3-101-A00001).
        assert!(validate("3101A00001").is_ok());
        assert!(validate_with("3101A00001", "cedula_juridica").is_ok());
    }

    #[test]
    fn rejects_leading_zeros() {
        // Anexos y Estructuras v4.4 forbids a leading zero only for the cédula
        // física and the DIMEX; the 10-digit NITE has no such rule.
        assert!(validate("012345678").is_err());
        assert!(validate("01234567890").is_err());
        assert!(validate("012345678901").is_err());
        assert!(validate("0123456789").is_ok()); // NITE or cédula jurídica
    }

    #[test]
    fn honours_the_nite_kind() {
        // "10 dígitos y sin guiones": the canonical NITE, not the 12-character
        // reporting field and not the invoice-key zero padding.
        assert!(validate_with("0123456789", "nite").is_ok());
        assert!(validate_with("1234567890", "nite").is_ok());
        assert!(validate_with("0-1234-56789", "nite").is_ok());
        // The 12-character municipal/invoice field is the 10-digit number
        // padded with two zeros; the padded value is not a document number.
        assert!(validate_with("001234567890", "nite").is_err());
        assert!(validate_with("12345678901", "nite").is_err());
        // A cédula jurídica is 10 characters but its type digit is never 0 and
        // its consecutive may contain letters.
        assert_eq!(
            validate_with("0123456789", "cedula_juridica"),
            Err(ValidationError::InvalidFormat)
        );
        assert!(validate_with("3101A00001", "nite").is_err());
    }

    #[test]
    fn rejects_the_invoice_key_padded_dimex() {
        // An 11-digit DIMEX is left-padded with one zero only to fill the
        // 50-character invoice key; the number itself has no leading zero, so
        // the padded 12-digit form is not a valid document.
        assert!(validate("012345678901").is_err());
        assert!(validate_with("012345678901", "dimex").is_err());
        assert!(validate_with("01234567890", "dimex").is_err());
    }

    #[test]
    fn accepts_the_separators_people_type() {
        assert!(validate("1-0234-0567").is_ok());
        assert!(validate("1 0234 0567").is_ok());
        assert!(validate("3-101-123456").is_ok());
        assert!(validate("1.0234.0567").is_ok());
    }

    #[test]
    fn rejects_bad_lengths() {
        assert!(validate("12345678").is_err());
        assert!(validate("1234567890123").is_err());
        assert!(validate_with("1234567890123", "dimex").is_err());
        // Cédula jurídica needs exactly 4 numeric characters up front.
        assert!(validate_with("310A123456", "cedula_juridica").is_err());
        assert!(validate_with("310112345", "cedula_juridica").is_err());
    }

    #[test]
    fn honours_document_kind() {
        assert!(validate_with("123456789", "id_card").is_ok());
        assert!(validate_with("123456789", "dimex").is_err());
        assert!(validate_with("123456789", "cedula_juridica").is_err());
        assert!(validate_with("12345678901", "dimex").is_ok());
        assert!(validate_with("12345678901", "id_card").is_err());
        assert!(validate_with("AB123456", "passport").is_ok());
        assert!(validate_with("AB123456", "id_card").is_err());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_id_card() {
        let v = validated("123456789");
        assert_eq!(v.kind, "id_card");
        let card = match v.components {
            Some(Components::CedulaFisica(c)) => c,
            _ => panic!("expected cedula fisica"),
        };
        assert_eq!(card.province, 1);
    }

    #[test]
    fn components_of_juridical() {
        let v = validated("3101A00001");
        assert_eq!(v.kind, "cedula_juridica");
        let j = match v.components {
            Some(Components::CedulaJuridica(c)) => c,
            _ => panic!("expected cedula juridica"),
        };
        assert_eq!(j.person_type, 3);
        assert_eq!(j.class, 101);
    }

    #[test]
    fn components_of_dimex() {
        let v = validated("12345678901");
        assert_eq!(v.kind, "dimex");
        let d = match v.components {
            Some(Components::Dimex(c)) => c,
            _ => panic!("expected dimex"),
        };
        assert_eq!(d.country_code, 123);
        let v = validated("123456789012");
        let d = match v.components {
            Some(Components::Dimex(c)) => c,
            _ => panic!("expected dimex"),
        };
        assert_eq!(d.country_code, 123);
    }

    #[test]
    fn components_none_for_nite_and_passport() {
        let v = validated("1234567890");
        assert_eq!(v.kind, "nite");
        assert_eq!(v.components, None);
        let v = validated("AB123456");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }
}
