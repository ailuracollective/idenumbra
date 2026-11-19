//! Mexico identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "MEX";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/components.rs");
    include!("validators/date.rs");
    include!("validators/curp.rs");
    include!("validators/rfc.rs");
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{components, validate, validate_with, Components, Curp, Rfc, Validation};

/// Zero-sized marker type implementing [`Validator`] for Mexico.
pub struct IdenumbraMEX;

impl Validator for IdenumbraMEX {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
    fn document_kinds() -> &'static [&'static str] {
        &["curp", "rfc"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every value below is synthetic, derived here from the algorithm: placeholder
    // name initials (GALA, ABC, GODE, ÑABC), fixed dates, and the check digit
    // computed locally. The two SAT generic RFCs are reserved by RMF 2026.

    #[test]
    fn accepts_curp_and_rfc() {
        assert!(validate("GALA850315HNERXN05").is_ok());
        assert!(validate("LOPJ850101HDFRRN01").is_ok());
        // Non-binary CURP: RENAPO issues `X` in the sex position.
        assert!(validate("GALA850315XDFRRN07").is_ok());
        // Post-2000 birth: position 17 is a letter A-J (2004 is a leap year).
        assert!(validate("GALA040229XDFRRNA7").is_ok());
        assert!(validate("ABC010101AB1").is_ok());
        assert!(validate("GODE560101AB0").is_ok());
        // Ñ and & are allowed in the name part of an RFC.
        assert!(validate("ÑABC010101ABA").is_ok());
        assert!(validate("A&BC010101CD9").is_ok());
        assert!(validate("Ñ&Z010101CD9").is_ok());
        // SAT generic RFCs are accepted without a valid check digit.
        assert!(validate("XAXX010101000").is_ok());
        assert!(validate("XEXX010101000").is_ok());
    }

    #[test]
    fn rejects_bad_curp() {
        assert!(validate("GALA850315HNERXN06").is_err());
        assert!(validate("GALA850315YDFRRN07").is_err()); // sex must be H/M/X
        assert!(validate("GALA850315HXXRRN02").is_err()); // unknown state
        assert!(validate("GALA850315HQORRN05").is_err()); // QO is not a CURP code
        assert!(validate("GALA850315HDFARN03").is_err()); // vowel, not a consonant
        assert!(validate("GALA851315HNERXN03").is_err()); // month 13

        // Position 17 is 0-9 or A-J only; the check digit is correct here.
        assert!(validate("GALA040229XDFRRNK7").is_err());
    }

    #[test]
    fn rejects_bad_rfc() {
        assert!(validate("ABC010101AB2").is_err());
        assert!(validate("ABC011317AB1").is_err()); // month 13
        assert!(validate("ÑABC010101ABB").is_err()); // wrong check digit
        assert!(validate("GODE560101").is_err()); // 10 characters: not a fiscal RFC
        assert!(validate_with("ABC010101AB1", "curp").is_err());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_curp() {
        let v = validated("GALA850315HNERXN05");
        assert_eq!(v.kind, "curp");
        let curp = match v.components {
            Some(Components::Curp(c)) => c,
            _ => panic!("expected curp"),
        };
        assert_eq!(curp.name_code, *b"GALA");
        assert_eq!(curp.birth_date, *b"850315");
        assert_eq!(curp.sex, b'H');
        assert_eq!(curp.state, *b"NE");
        assert_eq!(curp.internal_consonants, *b"RXN");
        assert_eq!(curp.century_marker, b'0');
        assert_eq!(curp.check_digit, 5);

        let v = validated("GALA040229XDFRRNA7");
        let curp = match v.components {
            Some(Components::Curp(c)) => c,
            _ => panic!("expected curp"),
        };
        assert_eq!(curp.century_marker, b'A');
        assert_eq!(curp.check_digit, 7);
    }

    #[test]
    fn components_of_rfc() {
        let v = validated("ABC010101AB1");
        assert_eq!(v.kind, "rfc");
        let rfc = match v.components {
            Some(Components::Rfc(r)) => r,
            _ => panic!("expected rfc"),
        };
        assert_eq!(rfc.initials, [b'A', b'B', b'C', 0]);
        assert_eq!(rfc.initials_len, 3);
        assert_eq!(rfc.birth_date, *b"010101");
        assert_eq!(rfc.check_char, 1);

        let v = validated("GODE560101AB0");
        let rfc = match v.components {
            Some(Components::Rfc(r)) => r,
            _ => panic!("expected rfc"),
        };
        assert_eq!(rfc.initials, *b"GODE");
        assert_eq!(rfc.initials_len, 4);
        assert_eq!(rfc.birth_date, *b"560101");
        assert_eq!(rfc.check_char, 0);

        let v = validated("XAXX010101000");
        let rfc = match v.components {
            Some(Components::Rfc(r)) => r,
            _ => panic!("expected rfc"),
        };
        assert_eq!(rfc.initials, *b"XAXX");
        assert_eq!(rfc.check_char, 0);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("GALA850315HNERXN06").is_err());
        assert!(validate_with("ABC010101AB1", "curp").is_err());
    }
}
