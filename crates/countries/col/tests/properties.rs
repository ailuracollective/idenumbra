//! Property-based tests for the Colombian cédula, NIT and passport validators.
//!
//! Cédulas and passports are structural formats: neither has a check digit.
//! The independent NIT helper uses a remainder lookup table rather than the
//! validator's conditional. This crate returns `Result<(), ValidationError>`
//! and exposes no component API, so there are no component objects to inspect;
//! the NIT properties instead pin its body/check-digit split and every body
//! length directly.

use idenumbra_col::{validate, validate_with, IdenumbraCOL};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NIT_SEPARATORS: [u8; 3] = *b".- ";
const NON_NUMERIC: [u8; 7] = *b"aZ_/#*+";
const NON_ALPHANUMERIC: [u8; 7] = *b"_=./#*+";

/// DIAN weights aligned from the left for bodies of one through eleven digits.
const NIT_WEIGHTS: [u32; 11] = [47, 43, 41, 37, 29, 23, 19, 17, 13, 7, 3];

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn nit_check_digit(body: &str) -> u8 {
    let sum: u32 = body
        .bytes()
        .zip(NIT_WEIGHTS[11 - body.len()..].iter())
        .map(|(digit, weight)| u32::from(digit - b'0') * weight)
        .sum();
    // Remainders 0 and 1 stay unchanged; the others are subtracted from 11.
    b"01987654321"[(sum % 11) as usize] - b'0'
}

fn nit_from_body(body: &[u8]) -> String {
    let body = digits_to_string(body);
    let check = nit_check_digit(&body);
    format!("{body}{check}")
}

/// 6-10 digit cédulas, excluding the all-zero placeholder.
fn citizen_id() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 6..=10)
        .prop_filter("all-zero cédula is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

/// Alphanumeric passports with at least one letter, so inference cannot treat
/// them as numeric cédulas.
fn passport() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_filter("inferred passport must contain a letter", |chars| {
            chars
                .iter()
                .any(|character| character.is_ascii_alphabetic())
        })
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

/// Non-placeholder NIT bodies of every supported length.
fn nit_body() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(0u8..=9, 1..=11)
        .prop_filter("all-zero NIT body is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
}

fn nit() -> impl Strategy<Value = String> {
    nit_body().prop_map(|body| nit_from_body(&body))
}

fn inferred_value() -> impl Strategy<Value = String> {
    prop_oneof![citizen_id(), passport()]
}

proptest! {
    #[test]
    fn citizen_id_round_trip(value in citizen_id()) {
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "citizen_id"), Ok(()));
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(()));
        prop_assert_eq!(IdenumbraCOL::validate(&value), Ok(()));
        prop_assert_eq!(IdenumbraCOL::validate_with(&value, "citizen_id"), Ok(()));
        prop_assert_eq!(IdenumbraCOL::validate_with(&value, "id_card"), Ok(()));
    }

    #[test]
    fn citizen_id_has_no_check_digit(value in citizen_id(), replacement in 0u8..=9) {
        let mut bytes = value.into_bytes();
        let last = bytes.len() - 1;
        bytes[last] = b'0' + replacement;
        let mutated = String::from_utf8(bytes).unwrap();
        prop_assume!(mutated.bytes().any(|byte| byte != b'0'));

        // Every final digit is equally valid: no checksum is defined for a CC.
        prop_assert_eq!(validate(&mutated), Ok(()));
        prop_assert_eq!(validate_with(&mutated, "citizen_id"), Ok(()));
        prop_assert_eq!(validate_with(&mutated, "id_card"), Ok(()));
    }

    #[test]
    fn passport_round_trip(value in passport()) {
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
        prop_assert_eq!(validate_with(&value, "citizen_id"), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&value, "id_card"), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(IdenumbraCOL::validate(&value), Ok(()));
        prop_assert_eq!(IdenumbraCOL::validate_with(&value, "passport"), Ok(()));
    }

    #[test]
    fn nit_round_trip(value in nit()) {
        let body = &value[..value.len() - 1];
        prop_assert_eq!(value.as_bytes()[value.len() - 1] - b'0', nit_check_digit(body));
        prop_assert_eq!(validate_with(&value, "nit"), Ok(()));
        prop_assert_eq!(IdenumbraCOL::validate_with(&value, "nit"), Ok(()));

        // Bare NITs are never inferred as NITs. Lengths accepted as cédulas
        // ignore the final check digit; all other numeric lengths are rejected.
        if (6..=10).contains(&value.len()) {
            prop_assert_eq!(validate(&value), Ok(()));
            prop_assert_eq!(validate_with(&value, "citizen_id"), Ok(()));
            prop_assert_eq!(validate_with(&value, "id_card"), Ok(()));
        } else {
            let length = Err(ValidationError::InvalidLength {
                expected: 10,
                found: value.len(),
            });
            prop_assert_eq!(validate(&value), length);
            prop_assert_eq!(validate_with(&value, "citizen_id"), Err(ValidationError::InvalidFormat));
            prop_assert_eq!(validate_with(&value, "id_card"), Err(ValidationError::InvalidFormat));
        }
    }

    #[test]
    fn changing_only_the_nit_check_digit_is_rejected(
        value in nit(),
        replacement in 0u8..=9,
    ) {
        let body = &value[..value.len() - 1];
        let expected = nit_check_digit(body);
        prop_assume!(replacement != expected);

        let invalid = format!("{body}{replacement}");
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate_with(&invalid, "nit"), mismatch);
        prop_assert_eq!(IdenumbraCOL::validate_with(&invalid, "nit"), mismatch);

        // In particular, a ten-digit NIT with a bad check digit is still a
        // structurally valid ten-digit cédula when no kind is supplied.
        if (6..=10).contains(&invalid.len()) {
            prop_assert_eq!(validate(&invalid), Ok(()));
        }
    }

    #[test]
    fn nit_separators_and_surrounding_whitespace_are_ignored(
        value in nit(),
        position in 0usize..=12,
        separator in prop::sample::select(&NIT_SEPARATORS[..]),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value.clone();
        input.insert(at, char::from(separator));
        prop_assert_eq!(validate_with(&input, "nit"), validate_with(&value, "nit"));

        let padded = format!(" \t{input}\n ");
        prop_assert_eq!(validate_with(&padded, "nit"), Ok(()));
    }

    #[test]
    fn nit_non_numeric_characters_report_their_index(
        value in nit(),
        position in 0usize..=12,
        character in prop::sample::select(&NON_NUMERIC[..]),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value;
        input.insert(at, char::from(character));
        prop_assert_eq!(
            validate_with(&input, "nit"),
            Err(ValidationError::NonNumeric { index: at })
        );
    }

    #[test]
    fn passport_non_alphanumeric_characters_report_their_index(
        value in passport(),
        position in 0usize..12,
        character in prop::sample::select(&NON_ALPHANUMERIC[..]),
    ) {
        let at = position % value.len();
        let mut bytes = value.into_bytes();
        bytes[at] = character;
        let input = String::from_utf8(bytes).unwrap();
        prop_assert_eq!(
            validate(&input),
            Err(ValidationError::NonAlphanumeric { index: at })
        );
        prop_assert_eq!(validate_with(&input, "passport"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn inferred_digit_strings_of_other_lengths_are_rejected(
        mut digits in prop::collection::vec(0u8..=9, 1..=16),
    ) {
        prop_assume!(!(6..=10).contains(&digits.len()));
        digits[0] = 1;
        let value = digits_to_string(&digits);
        let length = Err(ValidationError::InvalidLength {
            expected: 10,
            found: value.len(),
        });
        prop_assert_eq!(validate(&value), length);
        prop_assert_eq!(validate_with(&value, "citizen_id"), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&value, "id_card"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn nit_digit_strings_of_other_lengths_are_rejected(
        mut digits in prop::collection::vec(0u8..=9, 0..=15),
    ) {
        prop_assume!(!(2..=12).contains(&digits.len()));
        if !digits.is_empty() {
            digits[0] = 1;
        }
        let value = digits_to_string(&digits);
        let expected = if digits.len() <= 1 {
            ValidationError::InvalidLength {
                expected: 10,
                found: digits.len(),
            }
        } else {
            // Compaction stops at the first digit beyond the twelve-byte buffer.
            ValidationError::InvalidLength {
                expected: 12,
                found: 13,
            }
        };
        prop_assert_eq!(validate_with(&value, "nit"), Err(expected));
    }

    #[test]
    fn inferred_passport_lengths_are_enforced(len in 0usize..=16) {
        prop_assume!(len == 0 || !(6..=12).contains(&len));
        let value = if len == 0 {
            String::new()
        } else {
            format!("A{}", "1".repeat(len - 1))
        };
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 12,
                    found: len,
                })
            );
        }
        prop_assert_eq!(validate_with(&value, "passport"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn all_zero_placeholders_are_rejected(
        citizen_len in 6usize..=10,
        nit_body_len in 1usize..=11,
    ) {
        let citizen = "0".repeat(citizen_len);
        prop_assert_eq!(validate(&citizen), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&citizen, "citizen_id"), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&citizen, "id_card"), Err(ValidationError::InvalidFormat));
        // Passport has no all-zero placeholder rule of its own.
        prop_assert_eq!(validate_with(&citizen, "passport"), Ok(()));

        let nit = format!("{}0", "0".repeat(nit_body_len));
        prop_assert_eq!(validate_with(&nit, "nit"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in inferred_value()) {
        let padded = format!("  \t{value}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "citizen_id");
        let _ = validate_with(&value, "id_card");
        let _ = validate_with(&value, "nit");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraCOL::validate(&value);
        let _ = IdenumbraCOL::validate_with(&value, "nit");
    }
}

#[test]
fn documented_synthetic_vectors_are_accepted() {
    // These Tier 1 fixtures were derived locally from the published algorithm;
    // their seeds and provenance are recorded in the crate README.
    assert_eq!(validate("12345678"), Ok(()));
    for value in [
        "900.657.800-0",
        "9006578000",
        "987.654.321-7",
        "12345678-8",
        "1098765432-9",
        "10987654321-4",
    ] {
        assert_eq!(validate_with(value, "nit"), Ok(()), "{value}");
    }
}

#[test]
fn empty_and_separator_only_inputs_are_rejected() {
    assert_eq!(validate(""), Err(ValidationError::Empty));
    assert_eq!(validate(" \t\n "), Err(ValidationError::Empty));
    assert_eq!(
        validate_with(" .- ", "nit"),
        Err(ValidationError::InvalidLength {
            expected: 10,
            found: 0,
        })
    );
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "CC", "NIT", "Passport", "passport ", "nit "] {
        assert_eq!(
            validate_with("12345678", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
