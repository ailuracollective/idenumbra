//! Honduras identification validator.
//!
//! Validates DNI (13 digits), RTN (14 digits) and passport numbers.
//! Implements [`idenumbra_core::Validator`], so it can be used on its own or
//! registered in the orchestrator. Validated numbers can also be split into
//! their parts with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "HND";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rtn.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Rtn, Validation};

/// Zero-sized marker type implementing [`Validator`] for Honduras.
pub struct IdenumbraHND;

impl Validator for IdenumbraHND {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
}
