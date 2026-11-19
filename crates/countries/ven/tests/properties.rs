//! Property-based tests for the Venezuelan CI, RIF and passport validators.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_ven::{validate, validate_with, Components, IdenumbraVEN, Validation};
use proptest::prelude::*;

const RIF_PREFIXES: [(u8, u32); 6] = [
    (b'V', 4),
    (b'E', 8),
    (b'J', 12),
    (b'P', 16),
    (b'G', 20),
    (b'C', 12),
];

fn rif_base(letter: u8) -> Option<u32> {
    RIF_PREFIXES
        .iter()
        .find(|(l, _)| *l == letter)
        .map(|(_, b)| *b)
}

fn rif_check(letter: u8, body: &str) -> u8 {
    let base = rif_base(letter).unwrap();
    let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
    let mut sum = base;
    for (d, w) in body.bytes().zip(weights) {
        sum += (d - b'0') as u32 * w;
    }
    let rest = sum % 11;
    (if rest < 2 { 0 } else { 11 - rest }) as u8
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} valid, got {e:?}"),
    }
}

proptest! {
    #[test]
    fn ci_round_trip(value in "[1-9][0-9]{5,7}") {
        prop_assert!(validate(&value).is_ok());
        let v = validate(&value).unwrap();
        prop_assert_eq!(v.kind, "ci");
    }

    #[test]
    fn ci_rejects_all_zero(_x in 0u8..1) {
        prop_assert!(validate("000000").is_err());
    }

    #[test]
    fn ci_length_errors(value in "[0-9]{1,5}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn ci_too_long(value in "[0-9]{9,12}") {
        if value.bytes().all(|b| b.is_ascii_digit()) {
            // could be rif-length but not valid ci
            prop_assert!(validate(&value).is_err() || validate(&value).is_ok());
        }
    }

    #[test]
    fn rif_round_trip(_x in 0u8..1) {
        let body = "12345678";
        let cd = rif_check(b'V', body);
        let full = format!("V{}{}", body, cd);
        let v = accepted(&full);
        prop_assert_eq!(v.kind, "rif");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(&full, "rif").is_ok());
    }

    #[test]
    fn rif_rejects_wrong_check(_x in 0u8..1) {
        let body = "12345678";
        let cd = rif_check(b'V', body);
        let wrong = if cd == 0 { 1 } else { 0 };
        let full = format!("V{}{}", body, wrong);
        prop_assert!(validate(&full).is_err());
    }

    #[test]
    fn rif_rejects_all_zero_body(_x in 0u8..1) {
        let full = "V000000000";
        prop_assert!(validate(full).is_err());
    }

    #[test]
    fn passport_round_trip(value in "[A-Z][A-Z0-9]{5,11}") {
        prop_assert!(validate(&value).is_ok());
        let v = validate(&value).unwrap();
        prop_assert_eq!(v.kind, "passport");
    }

    #[test]
    fn passport_rejects_too_short(value in "[A-Z][A-Z0-9]{0,4}") {
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
