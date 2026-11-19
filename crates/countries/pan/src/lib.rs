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
