//! Brazil identification validator.
//!
//! Validates CPF (11 digits) and CNPJ (14 characters) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

#[cfg(test)]
use idenumbra_core::ValidationError;

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/components.rs");
    include!("validators/validate.rs");
    include!("validators/char_val.rs");
    include!("validators/cpf.rs");
    include!("validators/cnpj.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{components, validate, validate_with, Cnpj, Components, Cpf, Validation};

/// Zero-sized marker type implementing [`Validator`] for Brazil.
pub struct IdenumbraBRA;

impl idenumbra_core::Validator for IdenumbraBRA {
    const COUNTRY: &'static str = "BRA";
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, idenumbra_core::ValidationError> {
        validate(value)
    }
    fn validate_with(
        value: &str,
        kind: &str,
    ) -> Result<Self::Validation, idenumbra_core::ValidationError> {
        crate::validate_with(value, kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_cpf_and_cnpj() {
        assert!(validate("123.456.789-09").is_ok());
        assert!(validate("12345678909").is_ok());
        assert!(validate("12.345.678/0001-95").is_ok());
        // Alphanumeric CNPJ, official format from July 2026.
        assert!(validate("12.ABC.345/01DE-35").is_ok());
    }

    #[test]
    fn accepts_alphanumeric_cnpj_vectors() {
        // The worked example printed in the Receita Federal / Serpro DV manual:
        // weights 5..2 give 459 (DV 3), then 6..2 give 424 (DV 5).
        assert!(validate("12.ABC.345/01DE-35").is_ok());
        // Synthetic bodies generated locally with the published algorithm.
        assert!(validate("ZZZZZZZZZZZZ62").is_ok());
        assert!(validate("A1B2C3D4E5F668").is_ok());
        // Lowercase input is uppercased before the ASCII-48 valuation.
        assert!(validate("12.abc.345/01de-35").is_ok());
    }

    #[test]
    fn rejects_wrong_alphanumeric_cnpj_digits() {
        assert_eq!(
            validate("12.ABC.345/01DE-45"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 3,
                received: 4,
            })
        );
        assert_eq!(
            validate("12.ABC.345/01DE-34"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 5,
                received: 4,
            })
        );
        // Only A-Z letters: no accents and no enye.
        assert!(validate("12.ABÇ.345/01DE-35").is_err());
        assert!(validate("12.ABÑ.345/01DE-35").is_err());
        // The check digits remain numeric.
        assert!(validate("12.ABC.345/01DE-E5").is_err());
    }

    #[test]
    fn rejects_bad_check_digits_and_repeats() {
        assert!(validate("12345678908").is_err());
        assert!(validate("12345678000194").is_err());
        assert_eq!(
            validate("111.111.111-11"),
            Err(ValidationError::InvalidFormat)
        );
        assert!(validate("1234567890").is_err());
    }

    #[test]
    fn honours_document_kind() {
        assert!(validate_with("123.456.789-09", "cpf").is_ok());
        assert!(validate_with("123.456.789-09", "cnpj").is_err());
        assert!(validate_with("12.345.678/0001-95", "cnpj").is_ok());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_cpf() {
        let v = validated("123.456.789-09");
        assert_eq!(v.kind, "cpf");
        let card = match v.components {
            Some(Components::Cpf(c)) => c,
            _ => panic!("expected cpf"),
        };
        assert_eq!(card.region, 9);
        assert_eq!(card.check_digit_1, 0);
        assert_eq!(card.check_digit_2, 9);
    }

    #[test]
    fn components_of_cnpj() {
        let v = validated("12.345.678/0001-95");
        assert_eq!(v.kind, "cnpj");
        let cnpj = match v.components {
            Some(Components::Cnpj(c)) => c,
            _ => panic!("expected cnpj"),
        };
        assert_eq!(cnpj.root, *b"12345678");
        assert_eq!(cnpj.branch, 1);
        assert_eq!(cnpj.check_digit_1, 9);
        assert_eq!(cnpj.check_digit_2, 5);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("12345678908").is_err());
        let v = validated("12.ABC.345/01DE-35");
        assert_eq!(v.kind, "cnpj");
        assert!(v.components.is_some());
    }
}
