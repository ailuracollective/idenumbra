//! Property-based tests for the Dominican cédula and passport validators.
//!
//! Cédulas are generated locally from fictional ten-digit payloads. The check
//! digit helper applies the published left-to-right `1,2,1,2,...` weights
//! independently of the validator's reverse traversal. The crate exposes no
//! component extractor for either document kind.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_dom::{validate, validate_with, IdenumbraDOM};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_ALNUM: [u8; 5] = *b"@._+=";
const ID_CARD_SEPARATORS: [u8; 2] = *b"- ";

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn luhn_check_digit(payload: &str) -> u8 {
    let sum: u32 = payload
        .bytes()
        .enumerate()
        .map(|(index, byte)| {
            let mut digit = u32::from(byte - b'0');
            if index % 2 == 1 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            digit
        })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

/// A non-placeholder cédula derived from a fictional ten-digit payload.
fn cedula_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 10)
        .prop_filter("all-zero cédula is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| {
            let mut value = digits_to_string(&digits);
            let check = luhn_check_digit(&value);
            value.push(char::from(b'0' + check));
            value
        })
}

fn passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

/// A passport with a letter, so inference cannot read it as a cédula.
fn inferable_passport() -> impl Strategy<Value = String> {
    passport_value().prop_filter("inference needs a non-digit", |value| {
        value.bytes().any(|byte| byte.is_ascii_alphabetic())
    })
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![cedula_digits(), inferable_passport()]
}

proptest! {
    #[test]
    fn cedula_round_trip(value in cedula_digits()) {
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(()));
        prop_assert_eq!(IdenumbraDOM::validate(&value), Ok(()));
    }

    #[test]
    fn canonical_cedula_formatting_is_ignored(value in cedula_digits()) {
        let formatted = format!("{}-{}-{}", &value[..3], &value[3..10], &value[10..]);
        prop_assert_eq!(validate(&formatted), Ok(()));
        prop_assert_eq!(validate_with(&formatted, "id_card"), Ok(()));
    }

    #[test]
    fn changing_only_the_check_digit_is_rejected(
        value in cedula_digits(),
        replacement in 0u8..=9,
    ) {
        let expected = luhn_check_digit(&value[..10]);
        prop_assume!(replacement != expected);

        let mut invalid = value;
        invalid.replace_range(10..11, &replacement.to_string());
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate(&invalid), mismatch);
        prop_assert_eq!(validate_with(&invalid, "id_card"), mismatch);
    }

    #[test]
    fn body_mutations_change_the_expected_check_digit(
        value in cedula_digits(),
        position in 0usize..10,
        replacement in 0u8..=9,
    ) {
        let mut bytes = value.into_bytes();
        let received = bytes[10] - b'0';
        prop_assume!(replacement != bytes[position] - b'0');
        bytes[position] = b'0' + replacement;
        let invalid = String::from_utf8(bytes).unwrap();
        let expected = luhn_check_digit(&invalid[..10]);
        prop_assume!(expected != received);

        prop_assert_eq!(
            validate(&invalid),
            Err(ValidationError::CheckDigitMismatch {
                expected: u32::from(expected),
                received: u32::from(received),
            })
        );
    }

    #[test]
    fn cedula_separators_are_ignored(
        value in cedula_digits(),
        position in 0usize..=11,
        separator in prop::sample::select(&ID_CARD_SEPARATORS[..]),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value.clone();
        input.insert(at, char::from(separator));
        prop_assert_eq!(validate(&input), validate(&value));
        prop_assert_eq!(validate_with(&input, "id_card"), validate_with(&value, "id_card"));
    }

    #[test]
    fn passport_round_trip(value in passport_value()) {
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
    }

    #[test]
    fn passports_with_letters_are_inferred(value in inferable_passport()) {
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
        prop_assert_eq!(validate_with(&value, "id_card"), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(IdenumbraDOM::validate(&value), Ok(()));
    }

    #[test]
    fn numeric_passports_are_accepted_by_the_explicit_kind(
        digits in prop::collection::vec(0u8..=9, 6..=12),
    ) {
        let value = digits_to_string(&digits);
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
    }

    #[test]
    fn passport_lengths_outside_six_to_twelve_are_rejected(
        chars in prop::collection::vec(prop::sample::select(ALNUM), 0..=16),
    ) {
        prop_assume!(!(6..=12).contains(&chars.len()));
        let value: String = chars.into_iter().map(char::from).collect();
        prop_assert_eq!(
            validate_with(&value, "passport"),
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: value.len(),
            })
        );
    }

    #[test]
    fn passport_non_alphanumerics_report_their_byte_index(
        value in passport_value(),
        position in 0usize..=12,
        character in prop::sample::select(&NON_ALNUM[..]),
    ) {
        prop_assume!(value.len() < 12);
        let at = position % (value.len() + 1);
        let mut input = value;
        input.insert(at, char::from(character));
        let error = Err(ValidationError::NonAlphanumeric { index: at });
        prop_assert_eq!(validate(&input), error);
        prop_assert_eq!(validate_with(&input, "passport"), error);
    }

    #[test]
    fn explicit_id_card_rejects_non_separator_characters(
        value in cedula_digits(),
        position in 0usize..=11,
    ) {
        let at = position % (value.len() + 1);
        let mut input = value;
        input.insert(at, 'A');
        prop_assert_eq!(
            validate_with(&input, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn cedula_digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(digits.len() != 11);
        let value = digits_to_string(&digits);
        let expected = if value.is_empty() {
            Err(ValidationError::Empty)
        } else {
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: digits.len().min(12),
            })
        };
        prop_assert_eq!(validate(&value), expected.clone());
        prop_assert_eq!(validate_with(&value, "id_card"), expected);
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let padded = format!(" \t{value}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "id_card");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraDOM::validate(&value);
    }

    #[test]
    fn unknown_kinds_are_rejected(value in valid_value()) {
        for kind in ["", "ID_CARD", "Passport", "passport ", "cedula"] {
            prop_assert_eq!(
                validate_with(&value, kind),
                Err(ValidationError::InvalidFormat),
                "kind: {}",
                kind
            );
        }
    }
}

#[test]
fn all_zero_placeholders_are_rejected() {
    for value in ["00000000000", "000-0000000-0", "000 0000000 0"] {
        assert_eq!(
            validate(value),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
        assert_eq!(
            validate_with(value, "id_card"),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
    }
}
