//! Peru identification validator.
//!
//! Validates DNI (8 digits), RUC (11 digits) and passport/carné de
//! extranjería (6-12 alphanumerics) numbers. Implements
//! [`idenumbra_core::Validator`], so it can be used on its own or registered
//! in the orchestrator. Validated numbers can also be split into their parts
//! with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "PER";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Ruc, Validation};

/// Zero-sized marker type implementing [`Validator`] for Peru.
pub struct IdenumbraPER;

impl Validator for IdenumbraPER {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
    fn document_kinds() -> &'static [&'static str] {
        &["dni", "ruc", "ce", "passport"]
    }
}
