//! Dominican Republic identification validator.

#![no_std]

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraDOM, "DOM", &["id_card", "passport"]);
