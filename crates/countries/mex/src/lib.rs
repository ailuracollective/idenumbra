//! Mexico identification validator.

#![no_std]

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/date.rs");
    include!("validators/curp.rs");
    include!("validators/rfc.rs");
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraMEX, "MEX");

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
}
