//! Property-based tests for the Mexican CURP and RFC validators.

use idenumbra_mex::{validate, validate_with, Components, IdenumbraMEX, Validation};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} valid, got {e:?}"),
    }
}

const VALID_CURP: &str = "GALA850315HNERXN05";
const VALID_RFC: &str = "ABC010101AB1";

proptest! {
    #[test]
    fn curp_round_trip(_x in 0u8..1) {
        let v = accepted(VALID_CURP);
        prop_assert_eq!(v.kind, "curp");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(VALID_CURP, "curp").is_ok());
        prop_assert!(validate_with(VALID_CURP, "rfc").is_err());
        prop_assert_eq!(IdenumbraMEX::validate(VALID_CURP), Ok(v));
    }

    #[test]
    fn rfc_round_trip(_x in 0u8..1) {
        let v = accepted(VALID_RFC);
        prop_assert_eq!(v.kind, "rfc");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(VALID_RFC, "rfc").is_ok());
        prop_assert!(validate_with(VALID_RFC, "curp").is_err());
        prop_assert_eq!(IdenumbraMEX::validate(VALID_RFC), Ok(v));
    }

    #[test]
    fn curp_rejects_bad_check(_x in 0u8..1) {
        let bad = "GALA850315HNERXN06";
        prop_assert!(validate(bad).is_err());
    }

    #[test]
    fn rfc_rejects_bad_check(_x in 0u8..1) {
        let bad = "ABC010101AB2";
        prop_assert!(validate(bad).is_err());
    }

    #[test]
    fn sat_generic_rfcs_accepted(_x in 0u8..1) {
        prop_assert!(validate("XAXX010101000").is_ok());
        prop_assert!(validate("XEXX010101000").is_ok());
    }

    #[test]
    fn curp_length_errors(value in "[A-Z0-9]{1,17}") {
        // Only specific lengths are valid; random short strings should fail
        if value.len() == 18 || value.len() == 13 || value.len() == 12 { return Ok(()); }
        prop_assert!(validate(&value).is_err());
    }



    #[test]
    fn panic_resistance(value in ".*") {
        let _ = validate(&value);
    }
}

#[test]
fn empty_rejected() {
    assert_eq!(validate(""), Err(ValidationError::Empty));
}
