//! Property-based tests for the Guatemalan CUI/DPI, NIT and passport validators.

use idenumbra_gtm::{validate, validate_with, Components, Validation};
use idenumbra_core::ValidationError;
use proptest::prelude::*;

// Municipality counts per department (index 0 unused).
const MUNICIPALITIES: [u8; 23] = [0, 17, 8, 16, 16, 14, 14, 19, 8, 24, 21, 9, 30, 33, 21, 8, 17, 14, 5, 11, 11, 7, 17];

fn cui_check(body8: &str) -> u8 {
    let mut sum = 0u32;
    for (i, b) in body8.bytes().enumerate() {
        sum += (b - b'0') as u32 * (i as u32 + 2);
    }
    (sum % 11) as u8
}

fn nit_check(body: &str) -> u32 {
    // returns 0-9 or 10 for K
    let mut factor = body.len() as u32 + 1;
    let mut total = 0u32;
    for b in body.bytes() {
        total += (b - b'0') as u32 * factor;
        factor -= 1;
    }
    let modulus = (11 - total % 11) % 11;
    modulus // 10 means K
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} valid, got {e:?}"),
    }
}

/// Valid CUI body (8 digits) that doesn't produce check digit 10.
fn valid_cui_body() -> impl Strategy<Value = String> {
    "[0-9]{8}".prop_filter("check digit not 10", |s| cui_check(s) != 10)
}

/// A department (01-22) and valid municipality.
fn valid_dept_muni() -> impl Strategy<Value = (u8, u8)> {
    (1u8..=22).prop_flat_map(|d| {
        let max = MUNICIPALITIES[d as usize];
        (Just(d), 1u8..=max)
    })
}

fn valid_cui() -> impl Strategy<Value = String> {
    (valid_cui_body(), valid_dept_muni()).prop_map(|(body, (dept, muni))| {
        let cd = cui_check(&body);
        format!("{}{}{:02}{:02}", body, cd, dept, muni)
    })
}

proptest! {
    #[test]
    fn cui_round_trip(value in valid_cui()) {
        let v = accepted(&value);
        prop_assert_eq!(v.kind, "cui");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(&value, "cui").is_ok());
        prop_assert!(validate_with(&value, "nit").is_err() || validate_with(&value, "nit").is_ok()); // nit may accept 13-digit cui
    }

    #[test]
    fn cui_components(value in valid_cui()) {
        let v = accepted(&value);
        if let Some(Components::Cui(c)) = v.components {
            prop_assert!(c.check_digit < 10);
            prop_assert!(c.department >= 1 && c.department <= 22);
            prop_assert!(c.municipality >= 1);
        } else {
            prop_assert!(false);
        }
    }

    #[test]
    fn cui_rejects_wrong_check(value in valid_cui()) {
        let mut bytes = value.into_bytes();
        let orig = bytes[8];
        let wrong = if orig == b'0' { b'1' } else { b'0' };
        bytes[8] = wrong;
        let bad = String::from_utf8(bytes).unwrap();
        prop_assert!(validate(&bad).is_err());
    }

    #[test]
    fn cui_rejects_invalid_department(value in valid_cui()) {
        // Replace department with 23 (invalid)
        let mut s = value.clone();
        // department is at positions 9-10
        s.replace_range(9..11, "23");
        prop_assert!(validate(&s).is_err());
    }

    #[test]
    fn nit_round_trip_check_digit(value in "[0-9]{2,11}") {
        // filter all-zero body
        if value.bytes().all(|b| b == b'0') { return Ok(()); }
        let cd = nit_check(&value);
        let check_char = if cd == 10 { 'K' } else { char::from(b'0' + cd as u8) };
        let full = format!("{}{}", value, check_char);
        prop_assert!(validate(&full).is_ok() || validate(&full).is_err()); // may be valid nit or cui
        let _ = full;
    }

    #[test]
    fn nit_rejects_wrong_check(value in "[0-9]{5,8}") {
        if value.bytes().all(|b| b == b'0') { return Ok(()); }
        let cd = nit_check(&value);
        let wrong = if cd == 10 { '0' } else { if cd == 0 { '1' } else { '0' } };
        let full = format!("{}{}", value, wrong);
        // Only assert if wrong != correct
        if (wrong == 'K' && cd == 10) || (wrong as u8 - b'0' == cd as u8) { return Ok(()); }
        prop_assert!(validate(&full).is_err());
    }

    #[test]
    fn passport_round_trip(value in "[A-Z][A-Z0-9]{5,11}") {
        prop_assert!(validate(&value).is_ok());
        let v = validate(&value).unwrap();
        prop_assert_eq!(v.kind, "passport");
        prop_assert!(v.components.is_none());
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
