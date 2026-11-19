//! Property-based tests for the Nicaraguan cédula and passport validators.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_nic::{validate, validate_with, Components, IdenumbraNIC, Validation};
use proptest::prelude::*;

const LETTERS: &[u8; 23] = b"ABCDEFGHJKLMNPQRSTUVWXY";

fn check_letter(digits: &str) -> u8 {
    let mut rest: u32 = 0;
    for b in digits.bytes() {
        rest = (rest * 10 + (b - b'0') as u32) % 23;
    }
    LETTERS[rest as usize]
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} valid, got {e:?}"),
    }
}

const BASE_CEDULA: &str = "0010101900001N";

#[test]
fn cedula_base_valid() {
    assert!(validate(BASE_CEDULA).is_ok());
}

#[test]
fn empty_rejected() {
    assert_eq!(validate(""), Err(ValidationError::Empty));
}

proptest! {
    #[test]
    fn cedula_round_trip(_x in 0u8..1) {
        let v = accepted(BASE_CEDULA);
        prop_assert_eq!(v.kind, "id_card");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(BASE_CEDULA, "id_card").is_ok());
        prop_assert!(validate_with(BASE_CEDULA, "passport").is_err());
        prop_assert_eq!(IdenumbraNIC::validate(BASE_CEDULA), Ok(v));
    }

    #[test]
    fn cedula_components(_x in 0u8..1) {
        let v = accepted(BASE_CEDULA);
        let card = match v.components {
            Some(Components::Cedula(c)) => c,
            _ => { prop_assert!(false); return Ok(()); }
        };
        prop_assert_eq!(card.municipality, 1);
    }

    #[test]
    fn cedula_accepts_formatting(_x in 0u8..1) {
        let formatted = "001-010190-0001N";
        prop_assert!(validate(formatted).is_ok());
    }

    #[test]
    fn cedula_rejects_wrong_letter(letter_idx in 0usize..23) {
        let wrong = LETTERS[letter_idx];
        if wrong == b'N' { return Ok(()); }
        let bad = format!("{}{}", &BASE_CEDULA[..13], char::from(wrong));
        prop_assert!(validate(&bad).is_err());
    }

    #[test]
    fn cedula_rejects_bad_month(_x in 0u8..1) {
        // day 32 is invalid for any month
        let s = "0013201900001";
        let letter = check_letter(s);
        let full = format!("{}{}", s, char::from(letter));
        prop_assert!(validate(&full).is_err());
    }

    #[test]
    fn cedula_length_errors(value in "[0-9]{1,12}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn passport_round_trip(value in "[A-Z0-9]{6,12}") {
        prop_assert!(validate(&value).is_ok());
        let v = validate(&value).unwrap();
        prop_assert_eq!(v.kind, "passport");
        prop_assert!(v.components.is_none());
        prop_assert!(validate_with(&value, "passport").is_ok());
        prop_assert!(validate_with(&value, "id_card").is_err());
    }

    #[test]
    fn passport_rejects_too_short(value in "[A-Z0-9]{1,5}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn passport_rejects_too_long(value in "[A-Z0-9]{13,20}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn explicit_kind_mismatch(_x in 0u8..1) {
        prop_assert!(validate_with(BASE_CEDULA, "passport").is_err());
    }

    #[test]
    fn unknown_kind_rejected(_x in 0u8..1) {
        prop_assert!(validate_with(BASE_CEDULA, "nit").is_err());
    }

    #[test]
    fn panic_resistance(value in ".*") {
        let _ = validate(&value);
    }
}
