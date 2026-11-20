//! Brazil identification validator.
//!
//! Validates CPF (11 digits) and CNPJ (14 characters) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

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

/// Zero-sized marker type implementing [`idenumbra_core::Validator`] for Brazil.
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
    fn document_kinds() -> &'static [&'static str] {
        &["cpf", "cnpj"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Retained (not covered by proptest): authoritative synthetic vectors
    // from the Receita Federal / Serpro DV manual and locally derived fixtures.
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
    }
}
