//! Cuba identification validator (identity card).

#![no_std]

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/date.rs");
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraCUB, "CUB");
