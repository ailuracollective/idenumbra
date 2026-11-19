//! Property-based tests for the Uruguayan CI and RUT validators.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_ury::{validate, validate_with, Components, IdenumbraURY, Validation};
use proptest::prelude::*;

fn ci_check(body: &str) -> u8 {
    let mut padded = [b'0'; 7];
    padded[7 - body.len()..].copy_from_slice(body.as_bytes());
    let weights = [2u32, 9, 8, 7, 6, 3, 4];
    let sum: u32 = padded
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    ((10 - sum % 10) % 10) as u8
}

fn rut_check(digits: &str) -> u32 {
    let weights = [4u32, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..11]
        .bytes()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    let rest = sum % 11;
    if rest == 0 {
        0
    } else {
        11 - rest
    }
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} valid, got {e:?}"),
    }
}

proptest! {
    #[test]
    fn ci_round_trip(body in "[1-9][0-9]{5,6}") {
        let cd = ci_check(&body);
        let full = format!("{}{}", body, cd);
        let v = accepted(&full);
        prop_assert_eq!(v.kind, "ci");
        prop_assert!(v.components.is_some());
        prop_assert!(validate_with(&full, "ci").is_ok());
    }

    #[test]
    fn ci_rejects_wrong_check(body in "[1-9][0-9]{5,6}") {
        let cd = ci_check(&body);
        let wrong = if cd == 0 { 1 } else { 0 };
        let full = format!("{}{}", body, wrong);
        prop_assert!(validate(&full).is_err());
    }

    #[test]
    fn rut_round_trip(_x in 0u8..1) {
        // office 22, middle 001, body, check
        let body = "2200001001"; // 10 digits + need 11 before check
        // construct: first 2 = office, then 6 digits, then 001
        let rut_body = "22123456001"; // 11 digits
        let cd = rut_check(&format!("{}{}", rut_body, "0")); // need 11 digits
        // Actually rut needs exactly 11 digits before check = 12 total
        let full11 = "22123456001"; // 11 digits
        let cd = rut_check(full11);
        if cd >= 10 { return Ok(()); }
        let full = format!("{}{}", full11, cd);
        let v = accepted(&full);
        prop_assert_eq!(v.kind, "rut");
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
