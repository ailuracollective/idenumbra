//! Property-based tests for the Argentine DNI and CUIT/CUIL validators.
//! These are the only document kinds the crate implements, so no properties
//! are defined for passports.
//!
//! The check-digit helper below is an independent implementation of the
//! published convention, written as python-stdnum's table lookup
//! (`"012345678990"[11 - (sum % 11)]`) instead of the validator's `match`, so
//! generated expectations do not simply mirror the code under test. python-stdnum
//! additionally accepts all-zero DNIs and enforces a CUIT prefix whitelist; this
//! crate intentionally diverges on both (see the crate README).

use idenumbra_arg::{validate, validate_with, Components, IdenumbraARG, Validation};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

const WEIGHTS: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
const SEPARATORS: [u8; 4] = *b".-/ ";
const NON_NUMERIC: [u8; 7] = *b"aZ_=#*+";

fn check_digit(first_ten: &str) -> u32 {
    let sum: u32 = first_ten
        .bytes()
        .zip(WEIGHTS)
        .map(|(digit, weight)| u32::from(digit - b'0') * weight)
        .sum();
    u32::from(b"012345678990"[11 - (sum % 11) as usize] - b'0')
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

/// 7-8 digit DNIs, excluding the all-zero placeholder.
fn dni_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 7..=8)
        .prop_filter("all-zero DNI is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

/// Prefix and DNI body whose combination is not the all-zero placeholder.
fn cuit_parts() -> impl Strategy<Value = (u8, u32)> {
    (0u8..=99, 0u32..=99_999_999).prop_filter("all-zero CUIT is a placeholder", |(prefix, dni)| {
        *prefix != 0 || *dni != 0
    })
}

fn cuit_digits(prefix: u8, dni: u32) -> String {
    let first_ten = format!("{prefix:02}{dni:08}");
    format!("{first_ten}{}", check_digit(&first_ten))
}

fn valid_cuit() -> impl Strategy<Value = String> {
    cuit_parts().prop_map(|(prefix, dni)| cuit_digits(prefix, dni))
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(validation) => validation,
        Err(error) => panic!("expected {value:?} to be valid, got {error:?}"),
    }
}

proptest! {
    #[test]
    fn dni_round_trip(value in dni_digits()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "dni");
        prop_assert!(validation.components.is_none());
        prop_assert!(validate_with(&value, "dni").is_ok());
        prop_assert!(validate_with(&value, "cuit").is_err());
        prop_assert!(validate_with(&value, "cuil").is_err());
        prop_assert_eq!(IdenumbraARG::validate(&value), Ok(validation));
    }

    #[test]
    fn generated_cuit_is_accepted((prefix, dni) in cuit_parts()) {
        let value = cuit_digits(prefix, dni);
        let check = check_digit(&value[..10]) as u8;

        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "cuit");
        prop_assert_eq!(
            validation.components,
            Some(Components::Cuit(idenumbra_arg::Cuit {
                prefix,
                dni,
                check_digit: check,
            }))
        );

        // CUIT and CUIL share the format and checksum; only the reported kind differs.
        let cuil = validate_with(&value, "cuil");
        prop_assert_eq!(cuil.as_ref().map(|v| v.kind), Ok("cuil"));
        prop_assert_eq!(cuil.map(|v| v.components), Ok(validation.components));

        // An 11-digit number is not a DNI.
        prop_assert!(validate_with(&value, "dni").is_err());
    }

    #[test]
    fn changing_only_the_check_digit_is_rejected(
        (prefix, dni) in cuit_parts(),
        replacement in 0u8..=9,
    ) {
        // The computed digit depends only on the first ten digits, so any other
        // final digit must be reported as a checksum mismatch.
        let expected = check_digit(&format!("{prefix:02}{dni:08}")) as u8;
        prop_assume!(replacement != expected);

        let value = format!("{prefix:02}{dni:08}{replacement}");
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate(&value), mismatch);
        prop_assert_eq!(validate_with(&value, "cuit"), mismatch);
    }

    #[test]
    fn separators_are_ignored(
        value in valid_cuit(),
        position in 0usize..=11,
        separator in prop::sample::select(&SEPARATORS[..]),
    ) {
        // Dots, hyphens, slashes and spaces are stripped wherever they appear,
        // including around the number itself (trim).
        let mut input = value.clone();
        let at = position % (input.len() + 1);
        input.insert(at, char::from(separator));
        prop_assert_eq!(validate(&input), validate(&value));
        prop_assert_eq!(validate_with(&input, "cuit"), validate_with(&value, "cuit"));

        let padded = format!(" \t{input}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        // Determinism and panic resistance: no input may make the validators
        // panic or disagree with themselves.
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "dni");
        let _ = validate_with(&value, "cuit");
        let _ = validate_with(&value, "cuil");
        let _ = validate_with(&value, "passport");
        let _ = IdenumbraARG::validate(&value);
    }

    #[test]
    fn digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 7 | 8 | 11));
        let value = digits_to_string(&digits);
        match validate(&value) {
            Err(ValidationError::Empty) => prop_assert!(value.is_empty()),
            Err(ValidationError::InvalidLength { .. }) => prop_assert!(!value.is_empty()),
            other => prop_assert!(false, "expected a length error for {value:?}, got {other:?}"),
        }
    }

    #[test]
    fn non_numeric_characters_are_rejected(
        value in dni_digits(),
        position in 0usize..=8,
        character in prop::sample::select(&NON_NUMERIC[..]),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value.clone();
        input.insert(at, char::from(character));
        prop_assert_eq!(validate(&input), Err(ValidationError::NonNumeric { index: at }));
    }
}

#[test]
fn all_zero_placeholders_are_rejected() {
    for value in [
        "0000000",
        "00000000",
        "0.000.000",
        "00000000000",
        "00-00000000-0",
    ] {
        assert_eq!(
            validate(value),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
    }
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "passport", "DNI", "Cuil", "cuit "] {
        assert_eq!(
            validate_with("12345678", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
