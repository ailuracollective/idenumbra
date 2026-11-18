//! Guatemala identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "GTM";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/cui.rs");
    include!("validators/nit.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraGTM;

impl Validator for IdenumbraGTM {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = ();
    fn validate(value: &str) -> Result<(), ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
        crate::validate_with(value, kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_all_zero_placeholders() {
        // Placeholder values are never issued.
        assert!(validate("00000000").is_err());
        assert!(validate("0000000000000").is_err());
    }

    #[test]
    fn accepts_cui_and_nit() {
        assert!(validate("1234567890101").is_ok());
        assert!(validate("12345679").is_ok());
        assert!(validate("1023456K").is_ok());
    }

    #[test]
    fn accepts_the_last_department_and_the_registration_place() {
        // Decreto 39-2010 art. 4: consular, naturalization and foreign-resident
        // registrations use the department/municipality boxes "según lo
        // establezca el Directorio del RENAP"; Acuerdo 55-2014 assigns the
        // place where the inscription was made, i.e. an ordinary code.
        assert!(validate("1234567890101").is_ok()); // Guatemala, 01-01
        assert!(validate("1234567892217").is_ok()); // Jutiapa, 22-17 (last code)
        assert!(validate("1234567892218").is_err()); // no 22-18
    }

    #[test]
    fn rejects_bad_cui() {
        assert_eq!(
            validate("1234567880101"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 9,
                received: 8
            })
        );
        assert_eq!(
            validate("1234567892301"),
            Err(ValidationError::InvalidRegion { region: 23 })
        );
        assert_eq!(
            validate("1234567890001"),
            Err(ValidationError::InvalidRegion { region: 0 })
        );
        // Department 01 only has 17 municipalities.
        assert!(validate("1234567890118").is_err());
        // Escuintla has 14 (Sipacate, Decreto 4-2015) and Huehuetenango 33
        // (Petatán, Decreto 6-2015): the last code of each is accepted, the
        // next one is not.
        assert!(validate("1234567890514").is_ok());
        assert!(validate("1234567891333").is_ok());
        assert!(validate("1234567890515").is_err());
        assert!(validate("1234567891334").is_err());
        // Municipality 00 does not exist.
        assert!(validate("1234567890500").is_err());
    }

    #[test]
    fn rejects_cui_whose_check_digit_would_be_10() {
        // 10000007 * 2..9 sums to a remainder of 10, which is not a digit, so
        // SAT's rule (§5.1.4.4, sum * 10 mod 11) has nothing to print.
        assert_eq!(
            validate("1000000700101"),
            Err(ValidationError::UnrepresentableCheckDigit)
        );
        // The same body has no valid 9-digit CUI-prefix NIT either.
        assert!(validate("100000070").is_err());
    }

    #[test]
    fn accepts_twelve_digit_cui() {
        // SAT §5.1.4.1: a CUI has 12 or 13 digits; the 12-digit form is a
        // correlativo that lost its leading zero. 01234567 * 2..9 -> 9.
        assert!(validate_with("123456790101", "cui").is_ok());
        assert!(validate_with("0123456790101", "cui").is_ok());
        assert!(validate_with("123456790101", "dpi").is_ok());
        assert_eq!(
            validate_with("123456720101", "cui"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 9,
                received: 2
            })
        );
        // Escuintla has 14 municipalities.
        assert!(validate_with("123456790515", "cui").is_err());
        // The 12-digit form is a CUI in circulation: inference tries the CUI
        // layout first and falls back to the NIT rule.
        assert!(validate("123456790101").is_ok());
    }

    #[test]
    fn accepts_nit_edges() {
        // 11 - 0 -> 0; 10 -> K.
        assert!(validate("12345660").is_ok());
        assert!(validate("1023456K").is_ok());
        assert!(validate("1023456k").is_ok());
        // 9 digits: NIT built from the first 9 digits of a CUI (SAT, 2023-10-04).
        assert!(validate("123456789").is_ok());
        // SAT §5.1.1(d): the hyphen right before the check character is dropped.
        assert!(validate("1234567-9").is_ok());
        assert!(validate("1023456-K").is_ok());
        assert!(validate_with("1234567-9", "nit").is_ok());
    }

    #[test]
    fn depurates_the_printed_forms() {
        // SAT §5.1.2: the CUI is "normalmente separado en 3 grupos por medio de
        // 2 espacios en blanco (AAAA BBBBB CCCC)" and any non-digit is dropped.
        assert!(validate("1234 56789 0101").is_ok());
        assert!(validate_with("1234 56789 0101", "cui").is_ok());
        assert!(validate_with("1234 56789 0101", "dpi").is_ok());
        // §5.1.1: spaces and dots are dropped from the NIT, and the hyphen or
        // underscore right before the check character is dropped too.
        assert!(validate("1.234.567-9").is_ok());
        assert!(validate("1234 567-9").is_ok());
        assert!(validate("1234567_9").is_ok());
        assert!(validate("102.345.6K").is_ok());
        // A separator anywhere else is still rejected.
        assert_eq!(
            validate_with("1234-5679", "nit"),
            Err(ValidationError::NonAlphanumeric { index: 4 })
        );
    }

    #[test]
    fn rejects_bad_nit() {
        assert!(validate("12345678").is_err());
        assert!(validate("1234567K").is_err());
        assert!(validate("0000000").is_err());
        assert_eq!(
            validate("1234567-8"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 9,
                received: 8
            })
        );
        // Only the separator before the check character is dropped.
        assert!(validate("1234-5679").is_err());
    }

    #[test]
    fn cui_prefix_is_a_nit() {
        // A CUI and its first nine digits (the post-2023 NIT) agree because the
        // CUI uses the same weights read the other way round.
        for (cui, nit) in [
            ("1234567890101", "123456789"),
            ("3141592660101", "314159266"),
        ] {
            assert!(validate_with(cui, "cui").is_ok());
            assert!(validate(nit).is_ok());
            assert_eq!(&cui[..9], nit);
        }
    }

    #[test]
    fn accepts_passport() {
        assert!(validate("A1234567").is_ok());
        assert!(validate_with("A1234567", "passport").is_ok());
        assert_eq!(
            validate("A1234"),
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: 5
            })
        );
    }

    #[test]
    fn nit_kind_accepts_a_cui() {
        // Since 2023-10-04 the NIT of a natural person derives from the CUI.
        assert!(validate_with("1234567890101", "nit").is_ok());
        assert_eq!(
            validate_with("1234567880101", "nit"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 9,
                received: 8
            })
        );
    }
}
