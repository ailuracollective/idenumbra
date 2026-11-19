//! Property-based tests for the Honduran DNI, RTN and passport validators.
//!
//! All values are synthetic and generated locally from the documented
//! structural rules. Neither DNI nor RTN has a published check digit, so the
//! properties cover classification, structure, components and panic resistance
//! rather than checksum mutation. In particular, every RTN trailing digit is
//! accepted for a non-zero embedded DNI.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_hnd::{components, validate, validate_with, Components, IdenumbraHND, Validation};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_NUMERIC: [u8; 5] = *b"_=#*+";
const NON_ALPHANUMERIC: [u8; 5] = *b"_-=#+";

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn digits_to_u64(digits: &str) -> u64 {
    digits
        .bytes()
        .fold(0u64, |number, digit| number * 10 + u64::from(digit - b'0'))
}

/// Thirteen digits, excluding the all-zero placeholder.
fn dni_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 13)
        .prop_filter("all-zero DNI is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

/// A non-zero embedded DNI followed by any trailing digit.
fn rtn_digits() -> impl Strategy<Value = String> {
    (dni_digits(), 0u8..=9).prop_map(|(mut dni, trailing)| {
        dni.push(char::from(b'0' + trailing));
        dni
    })
}

/// Six to twelve ASCII alphanumerics with at least two letters. Requiring two
/// letters keeps inference on the passport path when one character is mutated.
fn passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_filter("passport inference needs letters", |characters| {
            characters
                .iter()
                .filter(|character| character.is_ascii_alphabetic())
                .count()
                >= 2
        })
        .prop_map(|characters| characters.into_iter().map(char::from).collect())
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![dni_digits(), rtn_digits(), passport_value()]
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
        prop_assert_eq!(validation.components, None);
        prop_assert_eq!(validate_with(&value, "dni"), Ok(validation));
        prop_assert_eq!(
            validate_with(&value, "rtn"),
            Err(ValidationError::InvalidLength { expected: 14, found: 13 })
        );
        prop_assert_eq!(IdenumbraHND::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraHND::validate_with(&value, "dni"), Ok(validation));
        prop_assert_eq!(components(&value), Ok(None));
    }

    #[test]
    fn rtn_round_trip(value in rtn_digits()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "rtn");

        let rtn = match validation.components {
            Some(Components::Rtn(rtn)) => rtn,
            _ => panic!("expected RTN components"),
        };
        prop_assert_eq!(rtn.embedded_dni, digits_to_u64(&value[..13]));
        prop_assert_eq!(rtn.trailing, value.as_bytes()[13] - b'0');

        prop_assert_eq!(validate_with(&value, "rtn"), Ok(validation));
        prop_assert_eq!(
            validate_with(&value, "dni"),
            Err(ValidationError::InvalidLength { expected: 13, found: 14 })
        );
        prop_assert_eq!(IdenumbraHND::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraHND::validate_with(&value, "rtn"), Ok(validation));
        prop_assert_eq!(components(&value), Ok(Some(Components::Rtn(rtn))));
    }

    #[test]
    fn every_rtn_trailing_digit_is_accepted(dni in dni_digits()) {
        // The SAR does not publish a verifier algorithm: the fourteenth digit
        // is structural data, not a checksum to recompute or enforce.
        for trailing in 0u8..=9 {
            let value = format!("{dni}{trailing}");
            prop_assert!(validate(&value).is_ok(), "{value:?}");
            prop_assert!(validate_with(&value, "rtn").is_ok(), "{value:?}");
        }
    }

    #[test]
    fn passport_round_trip(value in passport_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "passport");
        prop_assert_eq!(validation.components, None);
        prop_assert_eq!(validate_with(&value, "passport"), Ok(validation));
        prop_assert_eq!(IdenumbraHND::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraHND::validate_with(&value, "passport"), Ok(validation));
        prop_assert_eq!(components(&value), Ok(None));
    }

    #[test]
    fn explicit_passport_accepts_numeric_values(
        digits in prop::collection::vec(0u8..=9, 6..=12),
    ) {
        let value = digits_to_string(&digits);
        let validation = validate_with(&value, "passport");
        prop_assert_eq!(validation.as_ref().map(|v| v.kind), Ok("passport"));
        prop_assert_eq!(validation.map(|v| v.components), Ok(None));
        prop_assert_eq!(
            validate(&value),
            Err(ValidationError::InvalidLength {
                expected: 13,
                found: value.len(),
            })
        );
    }

    #[test]
    fn numeric_separators_are_ignored(
        value in prop_oneof![dni_digits(), rtn_digits()],
        position in 0usize..=14,
        separator in prop::sample::select(&[b'-', b' '][..]),
    ) {
        let at = position % (value.len() + 1);
        let kind = if value.len() == 13 { "dni" } else { "rtn" };
        let mut input = value.clone();
        input.insert(at, char::from(separator));
        prop_assert_eq!(validate(&input), validate(&value));
        prop_assert_eq!(validate_with(&input, kind), validate_with(&value, kind));
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let validation = accepted(&value);
        let padded = format!(" \t{value}\n ");
        prop_assert_eq!(validate(&padded), Ok(validation));
        prop_assert_eq!(validate_with(&padded, validation.kind), Ok(validation));
    }

    #[test]
    fn digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 13 | 14));
        let value = digits_to_string(&digits);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else if digits.len() > 14 {
            // The fixed fourteen-byte compaction buffer reports the first
            // excess digit rather than counting the entire input.
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength { expected: 14, found: 15 })
            );
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 13,
                    found: digits.len(),
                })
            );
        }
    }

    #[test]
    fn passport_lengths_outside_the_range_are_rejected(length in 1usize..=20) {
        prop_assume!(!(6..=12).contains(&length));
        let value = "A".repeat(length);
        let error = Err(ValidationError::InvalidLength {
            expected: 6,
            found: length,
        });
        prop_assert_eq!(validate(&value), error);
        prop_assert_eq!(validate_with(&value, "passport"), error);
    }

    #[test]
    fn non_numeric_characters_are_rejected(
        value in prop_oneof![dni_digits(), rtn_digits()],
        position in 0usize..=14,
        character in prop::sample::select(&NON_NUMERIC[..]),
    ) {
        let at = position % (value.len() + 1);
        let kind = if value.len() == 13 { "dni" } else { "rtn" };
        let mut input = value.clone();
        input.insert(at, char::from(character));
        let error = Err(ValidationError::NonNumeric { index: at });
        prop_assert_eq!(validate(&input), error);
        prop_assert_eq!(validate_with(&input, kind), error);
    }

    #[test]
    fn non_alphanumeric_passport_characters_are_rejected(
        value in passport_value(),
        position in 0usize..=11,
        character in prop::sample::select(&NON_ALPHANUMERIC[..]),
    ) {
        let at = position % value.len();
        let mut input = value.clone();
        input.replace_range(at..=at, &char::from(character).to_string());
        let error = Err(ValidationError::NonAlphanumeric { index: at });
        prop_assert_eq!(validate(&input), error);
        prop_assert_eq!(validate_with(&input, "passport"), error);
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "dni");
        let _ = validate_with(&value, "rtn");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraHND::validate(&value);
        let _ = IdenumbraHND::validate_with(&value, "dni");
        let _ = components(&value);
    }

    #[test]
    fn all_zero_placeholders_are_rejected(length in prop::sample::select(&[13usize, 14][..])) {
        let value = "0".repeat(length);
        let kind = if length == 13 { "dni" } else { "rtn" };
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&value, kind), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(components(&value), Err(ValidationError::InvalidFormat));
    }
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "DNI", "RTN", "PASSPORT", "Passport", "dni ", "nit"] {
        assert_eq!(
            validate_with("1234567890123", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
