//! Ecuador identification validator.
//!
//! Validates ID card (10 digits), RUC (13 digits) and passport (alphanumeric)
//! numbers. Implements [`idenumbra_core::Validator`], so it can be used on its
//! own or registered in the orchestrator. Validated numbers can also be split
//! into their parts with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "ECU";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/id_card.rs");
    include!("validators/passport.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{
    components, validate, validate_with, Components, IdCard, Province, Ruc, Taxpayer, Validation,
};

/// Zero-sized marker type implementing [`Validator`] for Ecuador.
pub struct IdenumbraECU;

impl Validator for IdenumbraECU {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
}
