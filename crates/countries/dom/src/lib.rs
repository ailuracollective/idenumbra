//! Dominican Republic identification validator.

#![no_std]

#[cfg(test)]
use idenumbra_core::ValidationError;

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraDOM, "DOM");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_all_zero_placeholders() {
        // Placeholder values are never issued.
        assert!(validate("00000000000").is_err());
    }

    #[test]
    fn accepts_formatted_id_card() {
        assert!(validate("12345678903").is_ok());
        assert!(validate("123-4567890-3").is_ok());
        // The explicit kind reads the same separators as inference.
        assert!(validate_with("12345678903", "id_card").is_ok());
        assert!(validate_with("123-4567890-3", "id_card").is_ok());
        // A passport is not an identity card.
        assert_eq!(
            validate_with("AB123456", "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn rejects_bad_luhn_digit() {
        assert_eq!(
            validate("12345678904"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 3,
                received: 4
            })
        );
        assert!(validate("1234567890").is_err());
    }
}
