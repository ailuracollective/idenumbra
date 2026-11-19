//! Bolivia identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "BOL";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{components, validate, validate_with, Components, Nit, Validation};

/// Zero-sized marker type implementing [`Validator`] for Bolivia.
pub struct IdenumbraBOL;

impl Validator for IdenumbraBOL {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
    fn document_kinds() -> &'static [&'static str] {
        &["id_card", "nit"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Retained (not covered by proptest): documented SIN-list compatibility
    // examples (control code 03 and a trailing zero) plus the explicit-NIT
    // length/all-zero gaps that proptest does not exercise.
    #[test]
    fn nit_control_code_compatibility() {
        // 03 appears in SIN supplier lists but is undocumented by the SIN.
        assert!(validate_with("1234567037", "nit").is_ok());
        assert!(validate_with("123456030", "nit").is_ok());
        // The SIAT page restricts the final digit to 1-9, but SIN lists
        // contain 0, so it is accepted as compatibility.
        assert!(validate_with("1234567010", "nit").is_ok());
        // Explicit-NIT length errors (proptest only covers inferred validate).
        assert_eq!(
            validate_with("12345678", "nit"),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: 8
            })
        );
        assert_eq!(
            validate_with("123456701234", "nit"),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: 12
            })
        );
        // 10-digit all-zero NIT (not in the proptest placeholder list).
        assert!(validate_with("0000000000", "nit").is_err());
    }
}
