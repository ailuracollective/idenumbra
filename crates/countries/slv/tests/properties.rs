//! Property-based tests for the Salvadoran DUI and NIT validators.

use idenumbra_slv::{validate, validate_with};
use idenumbra_core::ValidationError;
use proptest::prelude::*;

fn dui_check(body8: &str) -> u8 {
    let mut sum = 0u32;
    for (i, b) in body8.bytes().enumerate() {
        sum += (b - b'0') as u32 * (9 - i as u32);
    }
    ((10 - sum % 10) % 10) as u8
}

proptest! {
    #[test]
    fn dui_round_trip(body in "[1-9][0-9]{7}") {
        let cd = dui_check(&body);
        let full = format!("{}{}", body, cd);
        prop_assert!(validate(&full).is_ok());
        prop_assert!(validate_with(&full, "dui").is_ok());
    }

    #[test]
    fn dui_rejects_wrong_check(body in "[1-9][0-9]{7}") {
        let cd = dui_check(&body);
        let wrong = if cd == 0 { 1 } else { 0 };
        let full = format!("{}{}", body, wrong);
        prop_assert!(validate(&full).is_err());
    }

    #[test]
    fn dui_rejects_all_zero(_x in 0u8..1) {
        prop_assert!(validate("000000000").is_err());
    }

    #[test]
    fn dui_length_errors(value in "[0-9]{1,8}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn passport_round_trip(value in "[A-Z][A-Z0-9]{5,11}") {
        prop_assert!(validate(&value).is_ok());
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
