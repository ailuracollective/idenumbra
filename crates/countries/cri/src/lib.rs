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
    fn document_kinds() -> &'static [&'static str] {
        &["id_card", "cedula_juridica", "dimex", "nite", "passport"]
    }
}
