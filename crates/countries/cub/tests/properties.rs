//! Property-based tests for the Cuban identity card and passport validators.
//!
//! The carné has no public check-digit algorithm, so these properties cover
//! calendar structure, document inference, the deliberately unconstrained sex
//! and control digits, exact errors, and panic resistance. All identification
//! values are generated locally from fictional dates and arbitrary digits.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_cub::{validate, validate_with, IdenumbraCUB};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_ALNUM: [u8; 7] = *b"-_=#*+/";

#[derive(Clone, Copy, Debug)]
struct IdCardParts {
    year: u8,
    month: u8,
    day: u8,
    century: u8,
    serial: u8,
    sex: u8,
    control: u8,
}

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn full_year(year: u8, century: u8) -> u32 {
    let base = match century {
        9 => 1800,
        0..=5 => 1900,
        _ => 2000,
    };
    base + u32::from(year)
}

fn id_card(parts: IdCardParts) -> String {
    format!(
        "{:02}{:02}{:02}{}{:02}{}{}",
        parts.year, parts.month, parts.day, parts.century, parts.serial, parts.sex, parts.control
    )
}

/// A valid calendar date followed by arbitrary serial, sex and control digits.
fn id_card_parts() -> impl Strategy<Value = IdCardParts> {
    (0u8..=99, 0u8..=9, 1u8..=12)
        .prop_flat_map(|(year, century, month)| {
            let last_day = days_in_month(full_year(year, century), month);
            (
                Just(year),
                Just(month),
                1u8..=last_day,
                Just(century),
                0u8..=99,
                0u8..=9,
                0u8..=9,
            )
        })
        .prop_map(
            |(year, month, day, century, serial, sex, control)| IdCardParts {
                year,
                month,
                day,
                century,
                serial,
                sex,
                control,
            },
        )
}

fn id_card_value() -> impl Strategy<Value = String> {
    id_card_parts().prop_map(id_card)
}

fn id_card_with_invalid_month() -> impl Strategy<Value = IdCardParts> {
    id_card_parts()
        .prop_flat_map(|parts| (Just(parts), prop_oneof![Just(0u8), 13u8..=99]))
        .prop_map(|(mut parts, month)| {
            parts.month = month;
            parts
        })
}

fn id_card_with_invalid_day() -> impl Strategy<Value = IdCardParts> {
    id_card_parts()
        .prop_flat_map(|parts| {
            let last_day = days_in_month(full_year(parts.year, parts.century), parts.month);
            (Just(parts), prop_oneof![Just(0u8), (last_day + 1)..=99])
        })
        .prop_map(|(mut parts, day)| {
            parts.day = day;
            parts
        })
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

/// Alphanumeric passports with a letter, so inference cannot choose `id_card`.
fn inferred_passport_value() -> impl Strategy<Value = String> {
    passport_value().prop_filter("all-digit input is inferred as an identity card", |value| {
        value.bytes().any(|byte| byte.is_ascii_alphabetic())
    })
}

fn numeric_passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 6..=12).prop_map(|digits| digits_to_string(&digits))
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![id_card_value(), inferred_passport_value()]
}

proptest! {
    #[test]
    fn id_card_round_trip(parts in id_card_parts()) {
        let value = id_card(parts);
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(()));
        // An 11-digit carné also satisfies the structural passport rules.
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
        prop_assert_eq!(IdenumbraCUB::validate(&value), validate(&value));
        prop_assert_eq!(
            IdenumbraCUB::validate_with(&value, "id_card"),
            validate_with(&value, "id_card")
        );
    }

    #[test]
    fn sex_and_control_digits_do_not_constrain_validity(
        parts in id_card_parts(),
        sex in 0u8..=9,
        control in 0u8..=9,
    ) {
        let mut mutated = parts;
        mutated.sex = sex;
        mutated.control = control;
        let value = id_card(mutated);
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(()));
    }

    #[test]
    fn impossible_months_are_rejected(parts in id_card_with_invalid_month()) {
        let value = id_card(parts);
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn impossible_days_are_rejected(parts in id_card_with_invalid_day()) {
        let value = id_card(parts);
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn inferred_passport_round_trip(value in inferred_passport_value()) {
        prop_assert_eq!(validate(&value), Ok(()));
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
        prop_assert_eq!(IdenumbraCUB::validate(&value), validate(&value));
        prop_assert_eq!(
            IdenumbraCUB::validate_with(&value, "passport"),
            validate_with(&value, "passport")
        );
    }

    #[test]
    fn numeric_passports_require_an_explicit_kind(value in numeric_passport_value()) {
        prop_assert_eq!(validate_with(&value, "passport"), Ok(()));
        prop_assume!(value.len() != 11);
        prop_assert_eq!(
            validate(&value),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: value.len(),
            })
        );
    }

    #[test]
    fn id_cards_reject_non_digits(
        parts in id_card_parts(),
        position in 0usize..11,
        character in prop::sample::select(LETTERS),
    ) {
        let mut bytes = id_card(parts).into_bytes();
        bytes[position] = character;
        let input = String::from_utf8(bytes).unwrap();

        // Inference accepts the 11-character alphanumeric value as a passport,
        // while the explicit identity-card API rejects its non-digit character.
        prop_assert_eq!(validate(&input), Ok(()));
        prop_assert_eq!(
            validate_with(&input, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn passports_reject_non_alphanumeric_characters(
        value in passport_value(),
        position in 0usize..12,
        character in prop::sample::select(&NON_ALNUM[..]),
    ) {
        let at = position % value.len();
        let mut bytes = value.into_bytes();
        bytes[at] = character;
        let input = String::from_utf8(bytes).unwrap();
        let expected = Err(ValidationError::NonAlphanumeric { index: at });
        prop_assert_eq!(validate(&input), expected);
        prop_assert_eq!(
            validate_with(&input, "passport"),
            Err(ValidationError::NonAlphanumeric { index: at })
        );
    }

    #[test]
    fn numeric_values_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(digits.len() != 11);
        let value = digits_to_string(&digits);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 11,
                    found: value.len(),
                })
            );
        }
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidLength {
                expected: 11,
                found: value.len(),
            })
        );
    }

    #[test]
    fn passports_of_other_lengths_are_rejected(length in 0usize..=20) {
        prop_assume!(!(6..=12).contains(&length));
        let value = "A".repeat(length);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 6,
                    found: length,
                })
            );
        }
        prop_assert_eq!(
            validate_with(&value, "passport"),
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: length,
            })
        );
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let padded = format!(" \t{value}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
        prop_assert_eq!(IdenumbraCUB::validate(&padded), validate(&value));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "id_card");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraCUB::validate(&value);
        let _ = IdenumbraCUB::validate_with(&value, "id_card");
        let _ = IdenumbraCUB::validate_with(&value, "passport");
    }
}

#[test]
fn unknown_kinds_are_rejected() {
    // Synthetic value generated locally from 1990-01-01 and arbitrary digits.
    for kind in ["", "ID_CARD", "PASSPORT", "passport ", "carné"] {
        assert_eq!(
            validate_with("90010112345", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
