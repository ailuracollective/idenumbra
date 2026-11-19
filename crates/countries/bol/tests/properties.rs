//! Property-based tests for the Bolivian CI and NIT validators.
//!
//! CI has no check digit and NIT has no checksum either (its last three
//! digits are SIN control codes), so the properties below exercise structure,
//! classification, component extraction and panic resistance rather than any
//! digit mutation. The NIT expectations are built from the documented control
//! codes and observed length range, not from a (nonexistent) verifier.

use idenumbra_bol::{components, validate, validate_with, Components, IdenumbraBOL, Validation};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn parse_u64(s: &str) -> u64 {
    s.bytes().fold(0u64, |n, b| n * 10 + (b - b'0') as u64)
}

/// 5-8 digit CIs, excluding the all-zero placeholder.
fn ci_digits() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(0u8..=9, 5..=8).prop_filter("all-zero CI is a placeholder", |digits| {
        digits.iter().any(|digit| *digit != 0)
    })
}

fn ci_digits_string() -> impl Strategy<Value = String> {
    ci_digits().prop_map(|digits| digits_to_string(&digits))
}

/// 1-2 ASCII alphanumeric characters (a complement).
fn complement_chars() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 1..=2)
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

/// A CI followed by a single separator and a complement.
fn ci_with_complement() -> impl Strategy<Value = String> {
    (
        ci_digits(),
        prop::sample::select(&["-", " "][..]),
        complement_chars(),
    )
        .prop_map(|(digits, sep, comp)| format!("{}{}{}", digits_to_string(&digits), sep, comp))
}

/// A CI with a letter-leading complement and no separator.
fn ci_with_nosep_complement() -> impl Strategy<Value = String> {
    (ci_digits(), complement_chars())
        .prop_filter("complement must start with a letter", |(_, comp)| {
            comp.bytes()
                .next()
                .map(|b| b.is_ascii_alphabetic())
                .unwrap_or(false)
        })
        .prop_map(|(digits, comp)| format!("{}{}", digits_to_string(&digits), comp))
}

/// A structurally valid NIT of 9, 10 or 11 digits.
fn nit_value() -> impl Strategy<Value = String> {
    (9usize..=11).prop_flat_map(|len| {
        let embedded_len = len - 3;
        (
            prop::collection::vec(0u8..=9, embedded_len),
            prop::sample::select(&[1u8, 2, 3, 4][..]),
            0u8..=9,
        )
            .prop_map(move |(embedded, control, trailing)| {
                let mut s = digits_to_string(&embedded);
                s.push('0');
                s.push(char::from(b'0' + control));
                s.push(char::from(b'0' + trailing));
                s
            })
    })
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(validation) => validation,
        Err(error) => panic!("expected {value:?} to be valid, got {error:?}"),
    }
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![ci_digits_string(), nit_value()]
}

proptest! {
    #[test]
    fn ci_round_trip(value in ci_digits_string()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "id_card");
        prop_assert!(validation.components.is_none());
        prop_assert!(validate_with(&value, "id_card").is_ok());
        prop_assert!(validate_with(&value, "nit").is_err());
        prop_assert_eq!(IdenumbraBOL::validate(&value), Ok(validation));
        prop_assert_eq!(validate(&value), validate_with(&value, "id_card"));
    }

    #[test]
    fn ci_with_complement_is_accepted(value in ci_with_complement()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "id_card");
        prop_assert!(validation.components.is_none());
        prop_assert!(validate_with(&value, "id_card").is_ok());
        prop_assert_eq!(validate(&value), validate_with(&value, "id_card"));
    }

    #[test]
    fn ci_with_nosep_complement_is_accepted(value in ci_with_nosep_complement()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "id_card");
        prop_assert!(validation.components.is_none());
    }

    #[test]
    fn nit_round_trip(value in nit_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "nit");

        let nit = match validation.components {
            Some(Components::Nit(n)) => n,
            _ => panic!("expected nit components"),
        };

        let len = value.len();
        let embedded_len = len - 3;
        let expected_embedded = parse_u64(&value[..embedded_len]);
        let expected_control = value.as_bytes()[embedded_len + 1] - b'0';
        let expected_trailing = value.as_bytes()[len - 1] - b'0';

        prop_assert_eq!(nit.embedded_ci, expected_embedded);
        prop_assert_eq!(nit.control_code, expected_control);
        prop_assert_eq!(nit.trailing, expected_trailing);

        prop_assert!(validate_with(&value, "nit").is_ok());
        prop_assert!(validate_with(&value, "id_card").is_err());
        prop_assert_eq!(validate(&value), validate_with(&value, "nit"));
        prop_assert_eq!(components(&value), Ok(Some(Components::Nit(nit))));
    }

    #[test]
    fn nit_invalid_control_code_rejected(value in nit_value(), bad in prop::sample::select(&[0u8, 5, 6, 7, 8, 9])) {
        let mut chars: Vec<char> = value.chars().collect();
        let idx = chars.len() - 2;
        chars[idx] = char::from(b'0' + bad);
        let invalid: String = chars.into_iter().collect();
        prop_assert_eq!(validate(&invalid), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid, "nit"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn nit_missing_zero_rejected(value in nit_value(), nonzero in 1u8..=9) {
        let mut chars: Vec<char> = value.chars().collect();
        let idx = chars.len() - 3;
        chars[idx] = char::from(b'0' + nonzero);
        let invalid: String = chars.into_iter().collect();
        prop_assert_eq!(validate(&invalid), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid, "nit"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 5..=11));
        let value = digits_to_string(&digits);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else if digits.len() <= 4 {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 8,
                    found: digits.len(),
                })
            );
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength {
                    expected: 11,
                    found: digits.len(),
                })
            );
        }
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        let _ = validate_with(&value, "id_card");
        let _ = validate_with(&value, "nit");
        let _ = validate_with(&value, "passport");
        let _ = IdenumbraBOL::validate(&value);
        let _ = components(&value);
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let padded = format!("  \t{}\n ", value);
        prop_assert_eq!(validate(&padded), validate(&value));
        // trimmed explicit kinds agree with the trimmed input
        let _ = validate_with(&padded, "id_card");
        let _ = validate_with(&padded, "nit");
    }

    #[test]
    fn nit_components_are_consistent(value in nit_value()) {
        let v = accepted(&value);
        prop_assert_eq!(v.kind, "nit");
        prop_assert_eq!(components(&value), Ok(v.components));
    }
}

#[test]
fn all_zero_placeholders_are_rejected() {
    for value in [
        "00000",
        "00000000",
        "000000000",
        "00000000000",
        "0000000-1A",
    ] {
        assert_eq!(
            validate(value),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
    }
    assert_eq!(
        validate_with("0000000", "id_card"),
        Err(ValidationError::InvalidFormat)
    );
    assert_eq!(
        validate_with("0000000000", "nit"),
        Err(ValidationError::InvalidFormat)
    );
}

#[test]
fn nit_with_complement_is_rejected() {
    assert_eq!(
        validate("123456012-1A"),
        Err(ValidationError::InvalidFormat)
    );
    assert_eq!(
        validate_with("123456012-1A", "nit"),
        Err(ValidationError::InvalidFormat)
    );
}

#[test]
fn invalid_complements_are_rejected() {
    for value in [
        "1234567-ABC",
        "1234567-",
        "1234567-@",
        "1234567-1!",
        "1234567-",
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
    for kind in ["", "passport", "ID_CARD", "nit ", "ci"] {
        assert_eq!(
            validate_with("1234567", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}

#[test]
fn nit_unknown_kinds_rejected() {
    for kind in ["", "passport", "NIT", "nit "] {
        assert_eq!(
            validate_with("123456012", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
