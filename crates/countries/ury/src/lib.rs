//! Uruguay identification validator.

#![no_std]

#[cfg(test)]
use idenumbra_core::ValidationError;

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rut.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraURY, "URY");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_all_zero_placeholders() {
        // Placeholder values are never issued.
        assert!(validate("00000000").is_err());
    }

    #[test]
    fn accepts_ci_and_rut() {
        assert!(validate("1234567-2").is_ok());
        assert!(validate("12345672").is_ok());
        assert!(validate("21-999983-001-9").is_ok());
        assert!(validate("219999830019").is_ok());
        // 22 is the only prefix the DGI assigns since October 2024.
        assert!(validate("22-999983-001-6").is_ok());
    }

    #[test]
    fn pads_six_digit_ci_to_seven() {
        // Body 123456 pads to 0123456, sum 109, digit 1.
        assert!(validate("1234561").is_ok());
        assert!(validate("01234561").is_ok());
        assert_eq!(
            validate("1234562"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 1,
                received: 2
            })
        );
    }

    #[test]
    fn rejects_unrepresentable_rut_check_digit() {
        // These eleven digits leave remainder 1, so the digit would be 10 and
        // no RUT with them can exist.
        assert_eq!(
            validate("219999740019"),
            Err(ValidationError::UnrepresentableCheckDigit)
        );
    }

    #[test]
    fn rejects_bad_ci_and_rut() {
        assert_eq!(
            validate("12345673"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 2,
                received: 3
            })
        );
        assert!(validate("219999830018").is_err());
        assert_eq!(
            validate("239999830019"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        assert!(validate("219999830029").is_err()); // positions 9-11 are 001
    }
}
