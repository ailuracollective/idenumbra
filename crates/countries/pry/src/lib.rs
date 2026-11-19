//! Paraguay identification validator.

#![no_std]

#[cfg(test)]
use idenumbra_core::ValidationError;

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraPRY, "PRY");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_all_zero_placeholders() {
        // Placeholder values are never issued.
        assert!(validate("0000000").is_err());
        assert!(validate("00000000").is_err());
        assert!(validate_with("0000000", "ci").is_err());
    }

    #[test]
    fn accepts_ci_and_ruc() {
        assert!(validate("1234567").is_ok()); // CI, no check digit
        assert!(validate("80001234-8").is_ok()); // company RUC
        assert!(validate_with("1236", "ruc").is_ok()); // shortest RUC body
        assert!(validate_with("12345678", "ci").is_ok()); // CI by kind

        // The explicit kinds read the same separators as inference.
        assert!(validate_with("1.234.567", "ci").is_ok());
        assert!(validate_with("1.234.567", "id_card").is_ok());
    }

    #[test]
    fn rejects_bad_ruc() {
        assert_eq!(
            validate("80001234-9"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 8,
                received: 9
            })
        );
        assert!(validate("800532492").is_err());
        assert!(validate("1234567890").is_err());
        assert!(validate_with("123", "ruc").is_err()); // shorter than a RUC
    }
}
