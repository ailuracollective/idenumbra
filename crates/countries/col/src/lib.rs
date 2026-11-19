//! Colombia identification validator.

#![no_std]

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraCOL, "COL", &["id_card", "nit", "passport"]);
