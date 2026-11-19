//! Property-based tests for the Peruvian DNI, RUC and foreign-ID validators.
//!
//! Every generated value is synthetic and derived locally from the documented
//! formats. The RUC check-digit helper uses a remainder lookup table rather
//! than the validator's arithmetic expression, so generated expectations are
//! independent of the implementation under test.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_per::{components, validate, validate_with, Components, IdenumbraPER, Ruc, Validation};
use proptest::prelude::*;

const WEIGHTS: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
const RUC_PREFIXES: [u8; 4] = [10, 15, 17, 20];
const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_ALNUM: [u8; 5] = *b"-._*!";

fn check_digit(first_ten: &str) -> u8 {
    // SUNAT remainder: 0 -> 1, 1 -> 0, and n -> 11 - n otherwise.
    const DIGIT_BY_REMAINDER: [u8; 11] = [1, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let sum: u32 = first_ten
        .bytes()
        .zip(WEIGHTS)
        .map(|(digit, weight)| u32::from(digit - b'0') * weight)
        .sum();
    DIGIT_BY_REMAINDER[(sum % 11) as usize]
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn parse_u32(digits: &str) -> u32 {
    digits
        .bytes()
        .fold(0, |number, digit| number * 10 + u32::from(digit - b'0'))
}

/// Eight-digit DNIs, excluding the all-zero placeholder.
fn dni_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 8)
        .prop_filter("all-zero DNI is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

fn ruc_body() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(0u8..=9, 8)
}

fn ruc_parts() -> impl Strategy<Value = (u8, Vec<u8>)> {
    (prop::sample::select(&RUC_PREFIXES[..]), ruc_body())
}

fn ruc_digits(prefix: u8, body: &[u8]) -> String {
    let first_ten = format!("{prefix:02}{}", digits_to_string(body));
    format!("{first_ten}{}", check_digit(&first_ten))
}

fn valid_ruc() -> impl Strategy<Value = String> {
    ruc_parts().prop_map(|(prefix, body)| ruc_digits(prefix, &body))
}

/// Alphanumeric foreign IDs with at least one letter, so inference cannot
/// classify them as a numeric DNI or RUC.
fn foreign_id() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_filter("foreign ID must contain a letter", |chars| {
            chars.iter().any(u8::is_ascii_alphabetic)
        })
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![dni_digits(), valid_ruc(), foreign_id()]
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
        prop_assert_eq!(validate_with(&value, "dni"), Ok(validation));
        prop_assert!(validate_with(&value, "ruc").is_err());
        prop_assert_eq!(components(&value), Ok(None));
        prop_assert_eq!(IdenumbraPER::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraPER::validate_with(&value, "dni"), Ok(validation));
    }

    #[test]
    fn generated_ruc_is_accepted((prefix, body) in ruc_parts()) {
        let value = ruc_digits(prefix, &body);
        let validation = accepted(&value);
        let embedded_dni = if prefix == 10 {
            Some(parse_u32(&value[2..10]))
        } else {
            None
        };
        let expected = Ruc {
            prefix,
            embedded_dni,
            check_digit: check_digit(&value[..10]),
        };

        prop_assert_eq!(validation.kind, "ruc");
        prop_assert_eq!(validation.components, Some(Components::Ruc(expected)));
        prop_assert_eq!(validate_with(&value, "ruc"), Ok(validation));
        prop_assert!(validate_with(&value, "dni").is_err());
        prop_assert_eq!(components(&value), Ok(Some(Components::Ruc(expected))));
        prop_assert_eq!(IdenumbraPER::validate(&value), Ok(validation));
    }

    #[test]
    fn changing_only_the_ruc_check_digit_is_rejected(
        (prefix, body) in ruc_parts(),
        replacement in 0u8..=9,
    ) {
        let first_ten = format!("{prefix:02}{}", digits_to_string(&body));
        let expected = check_digit(&first_ten);
        prop_assume!(replacement != expected);
        let value = format!("{first_ten}{replacement}");
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });

        prop_assert_eq!(validate(&value), mismatch);
        prop_assert_eq!(validate_with(&value, "ruc"), mismatch);
    }

    #[test]
    fn unsupported_ruc_prefixes_are_rejected(
        (prefix, body) in (0u8..=99, ruc_body())
            .prop_filter("prefix must be unsupported", |(prefix, _)| {
                !RUC_PREFIXES.contains(prefix)
            }),
    ) {
        // Compute a valid check digit so only the prefix can cause rejection.
        let value = ruc_digits(prefix, &body);
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&value, "ruc"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn foreign_id_round_trip(value in foreign_id()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "passport");
        prop_assert!(validation.components.is_none());
        prop_assert_eq!(validate_with(&value, "passport"), Ok(validation));
        prop_assert_eq!(
            validate_with(&value, "ce"),
            Ok(Validation {
                kind: "ce",
                components: None,
            })
        );
        prop_assert!(validate_with(&value, "dni").is_err());
        prop_assert!(validate_with(&value, "ruc").is_err());
        prop_assert_eq!(components(&value), Ok(None));
        prop_assert_eq!(IdenumbraPER::validate(&value), Ok(validation));
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let validation = accepted(&value);
        let padded = format!(" \t{value}\n ");
        prop_assert_eq!(validate(&padded), Ok(validation));
        prop_assert_eq!(validate_with(&padded, validation.kind), Ok(validation));
        prop_assert_eq!(components(&padded), Ok(validation.components));
    }

    #[test]
    fn digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 8 | 11));
        let value = digits_to_string(&digits);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 8,
                    found: value.len(),
                })
            );
        }
    }

    #[test]
    fn explicit_foreign_ids_of_other_lengths_are_rejected(
        chars in prop::collection::vec(prop::sample::select(ALNUM), 0..=16),
    ) {
        prop_assume!(!(6..=12).contains(&chars.len()));
        let value: String = chars.into_iter().map(char::from).collect();
        prop_assert_eq!(
            validate_with(&value, "passport"),
            Err(ValidationError::InvalidFormat)
        );
        prop_assert_eq!(validate_with(&value, "ce"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn non_alphanumeric_characters_are_rejected(
        value in foreign_id(),
        position in 0usize..=12,
        character in prop::sample::select(&NON_ALNUM[..]),
    ) {
        let at = position % value.len();
        let mut bytes = value.into_bytes();
        bytes[at] = character;
        let input = String::from_utf8(bytes).unwrap();
        prop_assert_eq!(
            validate(&input),
            Err(ValidationError::NonAlphanumeric { index: at })
        );
        prop_assert_eq!(
            validate_with(&input, "passport"),
            Err(ValidationError::InvalidFormat)
        );
        prop_assert_eq!(validate_with(&input, "ce"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "dni");
        let _ = validate_with(&value, "ruc");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "ce");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraPER::validate(&value);
        let _ = IdenumbraPER::validate_with(&value, "ruc");
        let _ = components(&value);
    }
}

#[test]
fn all_zero_dni_placeholder_is_rejected() {
    assert_eq!(validate("00000000"), Err(ValidationError::InvalidFormat));
    assert_eq!(
        validate_with("00000000", "dni"),
        Err(ValidationError::InvalidFormat)
    );
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "DNI", "Ruc", "CE", "passport ", "tax_id"] {
        assert_eq!(
            validate_with("12345678", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
