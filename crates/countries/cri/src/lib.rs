//! Costa Rica identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "CRI";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraCRI;

impl Validator for IdenumbraCRI {
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
        // Placeholder values are never issued: every format forbids a leading
        // zero.
        assert!(validate("000000000").is_err());
        assert!(validate("00000000000").is_err());
        assert!(validate("0000000000").is_err());
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
        // Hacienda states every document is written without a leading zero.
        assert!(validate("012345678").is_err());
        assert!(validate("0123456789").is_err());
        assert!(validate("01234567890").is_err());
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
}
