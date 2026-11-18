//! Mexico identification validator.

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "MEX";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/date.rs");
    include!("validators/curp.rs");
    include!("validators/rfc.rs");
    include!("validators/validate.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

/// Country validator marker.
pub struct IdenumbraMEX;

impl Validator for IdenumbraMEX {
    const COUNTRY: &'static str = COUNTRY;
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
    fn accepts_curp_and_rfc() {
        assert!(validate("BOXW310820HNERXN09").is_ok());
        assert!(validate("LOPJ850101HDFRRN01").is_ok());
        // Non-binary CURP: RENAPO issues `X` in the sex position.
        assert!(validate("GALA850315XDFRRN07").is_ok());
        // Post-2000 birth: the 17th character is a letter (2004 is a leap year).
        assert!(validate("GALA040229XDFRRN07").is_ok());
        assert!(validate("EKU9003173C9").is_ok());
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
        assert!(validate("BOXW310820HNERXN08").is_err());
        assert!(validate("GALA850315YDFRRN07").is_err()); // sex must be H/M/X
        assert!(validate("GALA850315HXXRRN02").is_err()); // unknown state
        assert!(validate("GALA850315HQORRN05").is_err()); // QO is not a CURP code
        assert!(validate("GALA850315HDFARN03").is_err()); // vowel, not a consonant
        assert!(validate("BOXW311320HNERXN09").is_err()); // month 13
    }

    #[test]
    fn rejects_bad_rfc() {
        assert!(validate("EKU9003173C8").is_err());
        assert!(validate("EKU9013173C9").is_err()); // month 13
        assert!(validate("ÑABC010101ABB").is_err()); // wrong check digit
        assert!(validate("GODE560101").is_err()); // 10 characters: not a fiscal RFC
        assert!(validate_with("EKU9003173C9", "curp").is_err());
    }
}
