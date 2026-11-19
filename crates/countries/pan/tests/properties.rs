//! Property-based tests for the Panamanian cédula, RUC and passport validators.
//!
//! Cédulas and RUCs have no self-contained check digit here: the DGI's RUC DV
//! depends on taxpayer metadata outside this crate's API. These properties
//! therefore exercise structure, classification, component extraction and
//! panic resistance. All generated identifiers are synthetic and derived
//! locally from the documented shapes.

use idenumbra_core::{ValidationError, Validator};
use idenumbra_pan::{
    components, validate, validate_with, Cedula, Components, IdenumbraPAN, Validation,
};
use proptest::prelude::*;

const ID_MARKERS: [&str; 5] = ["E", "N", "PE", "AV", "PI"];
const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_ALNUM: [u8; 6] = *b"*+#_=!";

#[derive(Debug, Clone)]
struct CedulaCase {
    first: String,
    province: u8,
    marker: &'static str,
    folio: u32,
    asiento: u32,
}

impl CedulaCase {
    fn value(&self) -> String {
        format!("{}-{}-{}", self.first, self.folio, self.asiento)
    }

    fn components(&self) -> Option<Components> {
        let mut marker = [0u8; 2];
        marker[..self.marker.len()].copy_from_slice(self.marker.as_bytes());
        Some(Components::Cedula(Cedula {
            province: self.province,
            marker_len: self.marker.len() as u8,
            marker,
            folio: self.folio,
            asiento: self.asiento,
            is_nt: false,
        }))
    }
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

/// A three-part cédula with either a nonzero rollo or a valid marker prefix.
fn cedula_case() -> impl Strategy<Value = CedulaCase> {
    let rollo = 1u32..=999_999_999;
    let rollo = rollo.prop_map(|value| (value.to_string(), 0, ""));
    let marked = (
        prop::option::of((1u8..=13, any::<bool>())),
        prop::sample::select(&ID_MARKERS),
    )
        .prop_map(|(province, marker)| {
            let first = match province {
                Some((province, zero_padded)) if zero_padded => {
                    format!("{province:02}{marker}")
                }
                Some((province, _)) => format!("{province}{marker}"),
                None => marker.to_string(),
            };
            (first, province.map_or(0, |(province, _)| province), marker)
        });

    (prop_oneof![rollo, marked], 0u32..=9_999, 0u32..=999_999_999).prop_map(
        |((first, province, marker), folio, asiento)| CedulaCase {
            first,
            province,
            marker,
            folio,
            asiento,
        },
    )
}

/// Plain 7-13 digit cédulas/RUCs, excluding the all-zero placeholder.
fn plain_digits() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 7..=13)
        .prop_filter("all-zero plain values are placeholders", |digits| {
            digits.iter().any(|digit| *digit != 0)
        })
        .prop_map(|digits| digits_to_string(&digits))
}

/// Alphanumeric passports with at least one letter, so inference cannot
/// classify them as plain cédulas.
fn passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=12)
        .prop_filter("inferred passports contain a letter", |chars| {
            chars
                .iter()
                .any(|character| character.is_ascii_alphabetic())
        })
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

fn nt_parts() -> impl Strategy<Value = (u8, u32, u32)> {
    (1u8..=13, 0u32..=9_999, 0u32..=999_999_999)
}

fn invalid_province() -> impl Strategy<Value = u8> {
    (0u8..=99).prop_filter("outside provinces 1-13", |province| {
        *province == 0 || *province > 13
    })
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(validation) => validation,
        Err(error) => panic!("expected {value:?} to be valid, got {error:?}"),
    }
}

fn valid_value() -> impl Strategy<Value = String> {
    prop_oneof![
        cedula_case().prop_map(|case| case.value()),
        nt_parts()
            .prop_map(|(province, folio, asiento)| format!("{province}-NT-{folio}-{asiento}")),
        plain_digits(),
        passport_value(),
    ]
}

proptest! {
    #[test]
    fn cedula_round_trip(case in cedula_case()) {
        let value = case.value();
        let expected = Validation {
            kind: "id_card",
            components: case.components(),
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate(&value), Ok(expected));
        prop_assert_eq!(components(&value), Ok(expected.components));

        for kind in ["id_card", "cedula", "identity_card"] {
            let explicit = validate_with(&value, kind);
            prop_assert_eq!(explicit, Ok(expected));
            prop_assert_eq!(IdenumbraPAN::validate_with(&value, kind), explicit);
        }
        prop_assert_eq!(
            validate_with(&value, "ruc"),
            Ok(Validation {
                kind: "ruc",
                components: expected.components,
            })
        );
        prop_assert!(validate_with(&value, "passport").is_err());
    }

    #[test]
    fn nt_round_trip((province, folio, asiento) in nt_parts()) {
        let value = format!("{province}-NT-{folio}-{asiento}");
        let expected_components = Some(Components::Cedula(Cedula {
            province,
            marker_len: 0,
            marker: [0; 2],
            folio,
            asiento,
            is_nt: true,
        }));
        let expected = Validation {
            kind: "ruc",
            components: expected_components,
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate(&value), Ok(expected));
        prop_assert_eq!(validate_with(&value, "ruc"), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate_with(&value, "ruc"), Ok(expected));
        prop_assert_eq!(components(&value), Ok(expected_components));

        for kind in ["id_card", "cedula", "identity_card"] {
            prop_assert_eq!(
                validate_with(&value, kind),
                Err(ValidationError::InvalidFormat)
            );
        }
        prop_assert!(validate_with(&value, "passport").is_err());
    }

    #[test]
    fn plain_round_trip(value in plain_digits()) {
        let expected = Validation {
            kind: "id_card",
            components: None,
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate(&value), Ok(expected));
        prop_assert_eq!(components(&value), Ok(None));

        for kind in ["id_card", "cedula", "identity_card"] {
            prop_assert_eq!(validate_with(&value, kind), Ok(expected));
        }
        prop_assert_eq!(
            validate_with(&value, "ruc"),
            Ok(Validation {
                kind: "ruc",
                components: None,
            })
        );
        if value.len() <= 12 {
            prop_assert_eq!(
                validate_with(&value, "passport"),
                Ok(Validation {
                    kind: "passport",
                    components: None,
                })
            );
        } else {
            prop_assert_eq!(
                validate_with(&value, "passport"),
                Err(ValidationError::InvalidLength {
                    expected: 6,
                    found: value.len(),
                })
            );
        }
    }

    #[test]
    fn passport_round_trip(value in passport_value()) {
        let expected = Validation {
            kind: "passport",
            components: None,
        };
        prop_assert_eq!(validate(&value), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate(&value), Ok(expected));
        prop_assert_eq!(validate_with(&value, "passport"), Ok(expected));
        prop_assert_eq!(IdenumbraPAN::validate_with(&value, "passport"), Ok(expected));
        prop_assert_eq!(components(&value), Ok(None));

        for kind in ["id_card", "cedula", "identity_card", "ruc"] {
            prop_assert_eq!(
                validate_with(&value, kind),
                Err(ValidationError::InvalidFormat)
            );
        }
    }

    #[test]
    fn changing_plain_digits_does_not_create_a_checksum_requirement(
        value in plain_digits(),
        position in 0usize..=13,
        replacement in 0u8..=9,
    ) {
        let mut bytes = value.as_bytes().to_vec();
        let at = position % bytes.len();
        bytes[at] = b'0' + replacement;
        prop_assume!(bytes.iter().any(|byte| *byte != b'0'));
        let changed = String::from_utf8(bytes).unwrap();
        let validation = accepted(&changed);
        prop_assert_eq!(validation.kind, "id_card");
        prop_assert!(validation.components.is_none());
        prop_assert!(validate_with(&changed, "ruc").is_ok());
    }

    #[test]
    fn plain_digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 7..=13));
        prop_assume!(digits.is_empty() || digits.iter().any(|digit| *digit != 0));
        let value = digits_to_string(&digits);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else {
            let expected = Err(ValidationError::InvalidLength {
                expected: 10,
                found: value.len(),
            });
            prop_assert_eq!(validate(&value), expected);
            prop_assert_eq!(validate_with(&value, "id_card"), expected);
            prop_assert_eq!(validate_with(&value, "ruc"), expected);
        }
    }

    #[test]
    fn invalid_marked_provinces_are_rejected(province in invalid_province()) {
        let value = format!("{province}E-1-1");
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
        prop_assert_eq!(
            validate_with(&value, "ruc"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn invalid_nt_provinces_are_rejected(province in invalid_province()) {
        let value = format!("{province}-NT-1-1");
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "ruc"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn oversized_folios_are_rejected(folio in 10_000u32..=99_999) {
        let value = format!("8-{folio}-1");
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn oversized_asientos_are_rejected(asiento in 1_000_000_000u32..=u32::MAX) {
        let value = format!("8-1-{asiento}");
        prop_assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
    }

    #[test]
    fn passport_non_alphanumerics_are_rejected(
        value in passport_value(),
        position in 0usize..=12,
        character in prop::sample::select(&NON_ALNUM[..]),
    ) {
        let at = position % (value.len() + 1);
        let mut input = value.clone();
        input.insert(at, char::from(character));
        let expected = if input.len() > 12 {
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: input.len(),
            })
        } else {
            Err(ValidationError::NonAlphanumeric { index: at })
        };
        prop_assert_eq!(validate(&input), expected);
        prop_assert_eq!(validate_with(&input, "passport"), expected);
    }

    #[test]
    fn surrounding_whitespace_is_ignored(value in valid_value()) {
        let validation = accepted(&value);
        let padded = format!("  \t{value}\n ");
        prop_assert_eq!(validate(&padded), Ok(validation));
        prop_assert_eq!(components(&padded), Ok(validation.components));
        prop_assert_eq!(validate_with(&padded, validation.kind), Ok(validation));
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first, validate(&value));
        prop_assert_eq!(IdenumbraPAN::validate(&value), first);
        prop_assert_eq!(components(&value), first.map(|validation| validation.components));

        for kind in [
            "id_card",
            "cedula",
            "identity_card",
            "ruc",
            "passport",
            "unknown",
        ] {
            let explicit = validate_with(&value, kind);
            prop_assert_eq!(IdenumbraPAN::validate_with(&value, kind), explicit);
        }
    }
}

#[test]
fn all_zero_plain_placeholders_are_rejected() {
    for length in [7, 13] {
        let value = "0".repeat(length);
        assert_eq!(validate(&value), Err(ValidationError::InvalidFormat));
        assert_eq!(
            validate_with(&value, "id_card"),
            Err(ValidationError::InvalidFormat)
        );
        assert_eq!(
            validate_with(&value, "ruc"),
            Err(ValidationError::InvalidFormat)
        );
    }
}

#[test]
fn undocumented_series_are_rejected() {
    // EE and SB have no issuing rule, and E-NT is not a DGI NT shape.
    for value in ["8SB-1-234", "3SB-1-234", "EE-8-12345", "E-NT-8-123"] {
        assert_eq!(validate(value), Err(ValidationError::InvalidFormat));
    }
}

#[test]
fn malformed_parts_are_rejected() {
    for value in [
        "0-123-4567",
        "8-123",
        "AB-12-34",
        "8-12A-4567",
        "8-NTX-1-24",
        "8-NT-1-24-7",
    ] {
        assert_eq!(validate(value), Err(ValidationError::InvalidFormat));
        assert_eq!(components(value), Err(ValidationError::InvalidFormat));
    }
}

#[test]
fn unknown_kinds_are_rejected() {
    for kind in ["", "ID_CARD", "RUC", "Passport", "ruc ", "nit"] {
        assert_eq!(
            validate_with("1234567", kind),
            Err(ValidationError::InvalidFormat),
            "{kind}"
        );
    }
}
