//! Property-based tests for the Costa Rican validators.
//!
//! None of the numeric documents has a public check digit, so these properties
//! exercise structure, kind inference, component extraction and panic
//! resistance. Generated digits deliberately vary in every position rather
//! than using a checksum fixture. All concrete values are synthetic and were
//! authored here from the structural rules in the crate README.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_cri::{
    components, validate, validate_with, CedulaFisica, CedulaJuridica, Components, Dimex,
    IdenumbraCRI, Validation,
};
use proptest::prelude::*;

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const KINDS: &[&str] = &["id_card", "cedula_juridica", "nite", "dimex", "passport"];
const NON_ALPHANUMERIC: &[u8] = b"_=#*+/";
const SEPARATORS: &[u8] = b"-. ";

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn with_first(first: u8, rest: Vec<u8>) -> String {
    std::iter::once(first)
        .chain(rest)
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn alnum_to_string(chars: Vec<u8>) -> String {
    chars.into_iter().map(char::from).collect()
}

fn id_card_digits() -> impl Strategy<Value = String> {
    (1u8..=9, prop::collection::vec(0u8..=9, 8)).prop_map(|(first, rest)| with_first(first, rest))
}

fn dimex_digits() -> impl Strategy<Value = String> {
    (1u8..=9, 11usize..=12)
        .prop_flat_map(|(first, len)| (Just(first), prop::collection::vec(0u8..=9, len - 1)))
        .prop_map(|(first, rest)| with_first(first, rest))
}

fn nite_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 10)
        .prop_filter("all-zero NITE is a placeholder", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

fn juridical_value() -> impl Strategy<Value = String> {
    (
        1u8..=9,
        prop::collection::vec(0u8..=9, 3),
        prop::collection::vec(prop::sample::select(ALNUM), 6),
    )
        .prop_map(|(person_type, class, consecutive)| {
            format!(
                "{}{}{}",
                person_type,
                digits_to_string(&class),
                alnum_to_string(consecutive)
            )
        })
}

fn passport_value() -> impl Strategy<Value = String> {
    (6usize..=12)
        .prop_flat_map(|len| prop::collection::vec(prop::sample::select(ALNUM), len))
        .prop_map(alnum_to_string)
}

/// Alphanumeric values whose inference cannot collide with a numeric document
/// or with the special ten-character cédula jurídica branch.
fn inferred_passport_value() -> impl Strategy<Value = String> {
    prop::sample::select(&[6usize, 7, 8, 9, 11, 12][..])
        .prop_flat_map(|len| prop::collection::vec(prop::sample::select(ALNUM), len))
        .prop_map(alnum_to_string)
        .prop_filter("inferred passports contain a letter", |value| {
            value.bytes().any(|b| b.is_ascii_alphabetic())
        })
}

fn typed_value() -> impl Strategy<Value = (String, &'static str)> {
    prop_oneof![
        id_card_digits().prop_map(|value| (value, "id_card")),
        juridical_value().prop_map(|value| (value, "cedula_juridica")),
        nite_digits().prop_map(|value| (value, "nite")),
        dimex_digits().prop_map(|value| (value, "dimex")),
        passport_value().prop_map(|value| (value, "passport")),
    ]
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(validation) => validation,
        Err(error) => panic!("expected {value:?} to be valid, got {error:?}"),
    }
}

fn number(digits: &str) -> u32 {
    digits
        .bytes()
        .fold(0u32, |n, b| n * 10 + u32::from(b - b'0'))
}

fn inferred_ten_digit_kind(value: &str) -> &'static str {
    if (b'2'..=b'5').contains(&value.as_bytes()[0]) {
        "cedula_juridica"
    } else {
        "nite"
    }
}

fn valid_length(kind: &str, len: usize) -> bool {
    match kind {
        "id_card" => len == 9,
        "cedula_juridica" | "nite" => len == 10,
        "dimex" => matches!(len, 11 | 12),
        "passport" => (6..=12).contains(&len),
        _ => false,
    }
}

proptest! {
    #[test]
    fn id_card_round_trip(value in id_card_digits()) {
        let expected = Validation {
            kind: "id_card",
            components: Some(Components::CedulaFisica(CedulaFisica {
                province: value.as_bytes()[0] - b'0',
            })),
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate_with(&value, "id_card"), Ok(expected));
        prop_assert_eq!(components(&value), Ok(expected.components));
    }

    #[test]
    fn dimex_round_trip(value in dimex_digits()) {
        let expected = Validation {
            kind: "dimex",
            components: Some(Components::Dimex(Dimex {
                country_code: number(&value[..3]),
            })),
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(validate_with(&value, "dimex"), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate_with(&value, "dimex"), Ok(expected));
        prop_assert_eq!(components(&value), Ok(expected.components));
    }

    #[test]
    fn juridical_round_trip_and_inference(value in juridical_value()) {
        let juridical = Validation {
            kind: "cedula_juridica",
            components: Some(Components::CedulaJuridica(CedulaJuridica {
                person_type: value.as_bytes()[0] - b'0',
                class: number(&value[1..4]),
            })),
        };
        prop_assert_eq!(validate_with(&value, "cedula_juridica"), Ok(juridical));
        prop_assert_eq!(
            IdenumbraCRI::validate_with(&value, "cedula_juridica"),
            Ok(juridical)
        );

        let inferred = accepted(&value);
        let all_digits = value.bytes().all(|b| b.is_ascii_digit());
        if all_digits {
            prop_assert_eq!(inferred.kind, inferred_ten_digit_kind(&value));
        } else {
            prop_assert_eq!(inferred, juridical);
        }
        prop_assert_eq!(IdenumbraCRI::validate(&value), Ok(inferred));
        prop_assert_eq!(components(&value), Ok(inferred.components));
    }

    #[test]
    fn nite_explicit_kind_and_inference(value in nite_digits()) {
        let nite = Validation {
            kind: "nite",
            components: None,
        };
        prop_assert_eq!(validate_with(&value, "nite"), Ok(nite));
        prop_assert_eq!(IdenumbraCRI::validate_with(&value, "nite"), Ok(nite));

        let inferred = accepted(&value);
        prop_assert_eq!(inferred.kind, inferred_ten_digit_kind(&value));
        if inferred.kind == "nite" {
            prop_assert_eq!(inferred, nite);
        } else {
            prop_assert!(matches!(
                inferred.components,
                Some(Components::CedulaJuridica(_))
            ));
        }
        prop_assert_eq!(components(&value), Ok(inferred.components));
    }

    #[test]
    fn passport_explicit_kind(value in passport_value()) {
        let expected = Validation {
            kind: "passport",
            components: None,
        };
        prop_assert_eq!(validate_with(&value, "passport"), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate_with(&value, "passport"), Ok(expected));
    }

    #[test]
    fn inferred_passport_round_trip(value in inferred_passport_value()) {
        let expected = Validation {
            kind: "passport",
            components: None,
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(validate_with(&value, "passport"), Ok(expected));
        prop_assert_eq!(IdenumbraCRI::validate(&value), Ok(expected));
        prop_assert_eq!(components(&value), Ok(None));
    }

    #[test]
    fn separators_case_and_surrounding_whitespace_are_ignored(
        (value, kind) in typed_value(),
        position in 0usize..=12,
        separator in prop::sample::select(SEPARATORS),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value.clone();
        input.insert(at, char::from(separator));

        prop_assert_eq!(validate(&input), validate(&value));
        prop_assert_eq!(validate_with(&input, kind), validate_with(&value, kind));

        let lowercase = input.to_lowercase();
        prop_assert_eq!(validate(&lowercase), validate(&value));
        prop_assert_eq!(validate_with(&lowercase, kind), validate_with(&value, kind));

        let padded = format!(" \t{input}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
        prop_assert_eq!(validate_with(&padded, kind), validate_with(&value, kind));
    }

    #[test]
    fn leading_zero_id_card_and_dimex_are_rejected(
        mut value in prop_oneof![id_card_digits(), dimex_digits()],
    ) {
        value.replace_range(..1, "0");
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        let kind = if value.len() == 9 { "id_card" } else { "dimex" };
        prop_assert_eq!(validate_with(&value, kind), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn inferred_digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!(9..=12).contains(&digits.len()));
        let value = digits_to_string(&digits);
        let expected = if value.is_empty() {
            ValidationError::Empty
        } else if digits.len() > 12 {
            // Compact input occupies a twelve-byte buffer and reports the
            // thirteenth alphanumeric as soon as it is encountered.
            ValidationError::InvalidLength {
                expected: 12,
                found: 13,
            }
        } else {
            ValidationError::InvalidLength {
                expected: 9,
                found: digits.len(),
            }
        };
        prop_assert_eq!(validate(&value), Err(expected));
    }

    #[test]
    fn explicit_kinds_reject_other_lengths(
        digits in prop::collection::vec(0u8..=9, 0..=20),
        kind in prop::sample::select(KINDS),
    ) {
        prop_assume!(!valid_length(kind, digits.len()));
        let value = digits_to_string(&digits);
        let expected = if digits.len() > 12 {
            ValidationError::InvalidLength {
                expected: 12,
                found: 13,
            }
        } else {
            let expected = match kind {
                "id_card" => 9,
                "cedula_juridica" | "nite" => 10,
                "dimex" => 12,
                "passport" => 6,
                _ => unreachable!(),
            };
            ValidationError::InvalidLength {
                expected,
                found: digits.len(),
            }
        };
        prop_assert_eq!(validate_with(&value, kind), Err(expected));
    }

    #[test]
    fn non_alphanumeric_characters_report_their_byte_index(
        (value, kind) in typed_value(),
        position in 0usize..=12,
        character in prop::sample::select(NON_ALPHANUMERIC),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value;
        input.insert(at, char::from(character));
        let expected = Err(ValidationError::NonAlphanumeric { index: at });
        prop_assert_eq!(validate(&input), expected);
        prop_assert_eq!(validate_with(&input, kind), expected);
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        prop_assert_eq!(components(&value), first.map(|v| v.components));
        for kind in KINDS {
            prop_assert_eq!(
                IdenumbraCRI::validate_with(&value, kind),
                validate_with(&value, kind)
            );
        }
        let _ = IdenumbraCRI::validate(&value);
    }
}

#[test]
fn documented_synthetic_examples_are_accepted() {
    for (value, kind) in [
        ("123456789", "id_card"),
        ("1-0234-0567", "id_card"),
        ("3101123456", "cedula_juridica"),
        ("3101A00001", "cedula_juridica"),
        ("1234567890", "nite"),
        ("0123456789", "nite"),
        ("0-1234-56789", "nite"),
        ("12345678901", "dimex"),
        ("123456789012", "dimex"),
        ("AB123456", "passport"),
    ] {
        assert_eq!(
            validate_with(value, kind).map(|validation| validation.kind),
            Ok(kind),
            "{value}"
        );
    }

    assert_eq!(
        validate("3101A00001"),
        Ok(Validation {
            kind: "cedula_juridica",
            components: Some(Components::CedulaJuridica(CedulaJuridica {
                person_type: 3,
                class: 101,
            })),
        })
    );
    assert_eq!(validate("0123456789").map(|v| v.kind), Ok("nite"));
}

#[test]
fn all_zero_placeholders_are_rejected() {
    for (value, kind) in [
        ("000000000", "id_card"),
        ("0000000000", "cedula_juridica"),
        ("0000000000", "nite"),
        ("00000000000", "dimex"),
        ("000000000000", "dimex"),
    ] {
        assert_eq!(
            validate(value),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
        assert_eq!(
            validate_with(value, kind),
            Err(ValidationError::InvalidFormat),
            "{kind}: {value}"
        );
    }
}

#[test]
fn malformed_and_invoice_key_padded_values_are_rejected() {
    // A cédula jurídica needs four numeric characters up front.
    for value in ["310A123456", "0123456789", "310112345"] {
        assert_eq!(
            validate_with(value, "cedula_juridica"),
            if value.len() == 10 {
                Err(ValidationError::InvalidFormat)
            } else {
                Err(ValidationError::InvalidLength {
                    expected: 10,
                    found: 9,
                })
            },
            "{value}"
        );
    }

    // Invoice-key padding is not part of the document number.
    for value in ["01234567890", "012345678901"] {
        assert_eq!(
            validate(value),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
        assert_eq!(
            validate_with(value, "dimex"),
            Err(ValidationError::InvalidFormat),
            "{value}"
        );
    }
    assert!(validate_with("001234567890", "nite").is_err());
    assert!(validate_with("12345678901", "nite").is_err());
    assert!(validate_with("3101A00001", "nite").is_err());
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "ID_CARD", "Cedula_Juridica", "NITE", "passport ", "ci"] {
        assert_eq!(
            validate_with("123456789", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
