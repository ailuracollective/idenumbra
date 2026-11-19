//! Property-based tests for the Paraguayan CI and RUC validators.

use idenumbra_pry::{validate, validate_with};
use idenumbra_core::ValidationError;
use proptest::prelude::*;

fn ruc_check(body: &str) -> u32 {
    let mut sum = 0u32;
    let mut weight = 2u32;
    for d in body.bytes().rev() {
        sum += (d - b'0') as u32 * weight;
        weight = if weight == 11 { 2 } else { weight + 1 };
    }
    let rest = sum % 11;
    if rest > 1 { 11 - rest } else { 0 }
}

proptest! {
    #[test]
    fn ci_round_trip(value in "[1-9][0-9]{5,6}") {
        prop_assert!(validate(&value).is_ok());
        prop_assert!(validate_with(&value, "ci").is_ok());
        prop_assert!(validate_with(&value, "id_card").is_ok());
    }

    #[test]
    fn ci_rejects_all_zero(_x in 0u8..1) {
        prop_assert!(validate("0000000").is_err());
        prop_assert!(validate("00000000").is_err());
    }

    #[test]
    fn ruc_round_trip(body in "[1-9][0-9]{2,7}") {
        let cd = ruc_check(&body);
        let full = format!("{}{}", body, cd);
        prop_assert!(validate_with(&full, "ruc").is_ok());
    }

    #[test]
    fn ruc_rejects_wrong_check(body in "[1-9][0-9]{2,7}") {
        let cd = ruc_check(&body);
        let wrong = if cd == 0 { 1 } else { 0 };
        let full = format!("{}{}", body, wrong);
        prop_assert!(validate_with(&full, "ruc").is_err());
    }

    #[test]
    fn ci_ruc_length_rejected(value in "[0-9]{3,5}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn ci_ruc_too_long(value in "[0-9]{10,12}") {
        prop_assert!(validate(&value).is_err());
    }

    #[test]
    fn passport_round_trip(value in "[A-Z0-9]{6,12}") {
        // must contain a letter to be passport
        if !value.bytes().any(|b| b.is_ascii_alphabetic()) { return Ok(()); }
        prop_assert!(validate(&value).is_ok());
    }

    #[test]
    fn passport_rejects_too_short(value in "[A-Z0-9]{1,5}") {
        if !value.bytes().any(|b| b.is_ascii_alphabetic()) { return Ok(()); }
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
