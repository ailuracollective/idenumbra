//! Property-based tests for Ecuadorian cédula, RUC and passport validators.
//!
//! Every generated identification number is synthetic and derived locally from
//! fictional seeds. The modulo 10 and modulo 11 helpers independently implement
//! the published weights; private-company RUCs deliberately receive only their
//! structural checks because their final sequential digit is not a reliable
//! check digit.

use idenumbra_ecu::{
    components, validate, validate_with, Components, IdenumbraECU, Taxpayer, Validation,
};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

const ID_PROVINCES: [u8; 25] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    30,
];
const RUC_PROVINCES: [u8; 26] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    30, 50,
];
const ID_WEIGHTS: [u32; 9] = [2, 1, 2, 1, 2, 1, 2, 1, 2];
const PUBLIC_WEIGHTS: [u32; 8] = [3, 2, 7, 6, 5, 4, 3, 2];
const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NON_ALNUM: [u8; 6] = *b"_-=#*+";

fn id_check_digit(first_nine: &str) -> u8 {
    let sum: u32 = first_nine
        .bytes()
        .zip(ID_WEIGHTS)
        .map(|(digit, weight)| {
            let product = u32::from(digit - b'0') * weight;
            if product > 9 {
                product - 9
            } else {
                product
            }
        })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

/// A remainder of one would require check digit 10, which cannot be encoded.
fn public_check_digit(first_eight: &str) -> Option<u8> {
    let sum: u32 = first_eight
        .bytes()
        .zip(PUBLIC_WEIGHTS)
        .map(|(digit, weight)| u32::from(digit - b'0') * weight)
        .sum();
    let remainder = sum % 11;
    (remainder != 1).then(|| ((11 - remainder) % 11) as u8)
}

fn number(digits: &str) -> u32 {
    digits
        .bytes()
        .fold(0u32, |acc, digit| acc * 10 + u32::from(digit - b'0'))
}

fn digits_to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

fn replace_digit(value: &str, position: usize, replacement: u8) -> String {
    value
        .bytes()
        .enumerate()
        .map(|(index, digit)| {
            if index == position {
                char::from(b'0' + replacement)
            } else {
                char::from(digit)
            }
        })
        .collect()
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(validation) => validation,
        Err(error) => panic!("expected {value:?} to be valid, got {error:?}"),
    }
}

fn id_province() -> impl Strategy<Value = u8> {
    prop::sample::select(&ID_PROVINCES[..])
}

fn ruc_province() -> impl Strategy<Value = u8> {
    prop::sample::select(&RUC_PROVINCES[..])
}

fn id_card_value() -> impl Strategy<Value = String> {
    (id_province(), 0u8..=5, 0u32..=999_999).prop_map(|(province, person_type, serial)| {
        let body = format!("{province:02}{person_type}{serial:06}");
        format!("{body}{}", id_check_digit(&body))
    })
}

fn natural_ruc_value() -> impl Strategy<Value = String> {
    id_card_value().prop_map(|id_card| format!("{id_card}001"))
}

fn public_ruc_value() -> impl Strategy<Value = String> {
    (ruc_province(), 0u32..=99_999).prop_filter_map(
        "public RUC must have a representable check digit",
        |(province, serial)| {
            let body = format!("{province:02}6{serial:05}");
            public_check_digit(&body).map(|check| format!("{body}{check}0001"))
        },
    )
}

/// The seven digits after taxpayer type 9 are intentionally unconstrained.
fn private_ruc_value() -> impl Strategy<Value = String> {
    (ruc_province(), 0u64..=9_999_999)
        .prop_map(|(province, sequential)| format!("{province:02}9{sequential:07}001"))
}

fn numeric_ruc_value() -> impl Strategy<Value = String> {
    prop_oneof![
        natural_ruc_value(),
        public_ruc_value(),
        private_ruc_value(),
    ]
}

fn passport_value() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 6..=9)
        .prop_filter("numeric strings are inferred as malformed cédulas", |chars| {
            chars.iter().any(u8::is_ascii_alphabetic)
        })
        .prop_map(|chars| chars.into_iter().map(char::from).collect())
}

fn any_valid_value() -> impl Strategy<Value = String> {
    prop_oneof![
        id_card_value(),
        numeric_ruc_value(),
        passport_value(),
    ]
}

proptest! {
    #[test]
    fn id_card_round_trip_and_components(value in id_card_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "id_card");
        prop_assert_eq!(validate_with(&value, "id_card"), Ok(validation));
        prop_assert_eq!(validate_with(&value, "identity_card"), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate_with(&value, "id_card"), Ok(validation));
        prop_assert!(validate_with(&value, "ruc").is_err());
        prop_assert!(validate_with(&value, "passport").is_err());

        match validation.components {
            Some(Components::IdCard(card)) => {
                let province = number(&value[..2]);
                prop_assert_eq!(card.province.code, province);
                prop_assert!(card.province.name.is_some());
                prop_assert_eq!(card.person_type, value.as_bytes()[2] - b'0');
                prop_assert_eq!(card.serial, number(&value[3..9]));
                prop_assert_eq!(card.check_digit, value.as_bytes()[9] - b'0');
            }
            other => prop_assert!(false, "expected ID card components, got {other:?}"),
        }
        prop_assert_eq!(components(&value), Ok(validation.components));
    }

    #[test]
    fn natural_ruc_round_trip_and_components(value in natural_ruc_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "ruc");
        prop_assert_eq!(validate_with(&value, "ruc"), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate_with(&value, "ruc"), Ok(validation));
        prop_assert!(validate_with(&value, "id_card").is_err());

        match validation.components {
            Some(Components::Ruc(ruc)) => {
                prop_assert_eq!(ruc.taxpayer, Taxpayer::NaturalPerson);
                prop_assert_eq!(ruc.province.code, number(&value[..2]));
                prop_assert!(ruc.province.name.is_some());
                prop_assert_eq!(ruc.serial, None);
                let card = ruc.id_card.unwrap_or_else(|| panic!("expected embedded ID card"));
                prop_assert_eq!(card.province, ruc.province);
                prop_assert_eq!(card.person_type, value.as_bytes()[2] - b'0');
                prop_assert_eq!(card.serial, number(&value[3..9]));
                prop_assert_eq!(card.check_digit, value.as_bytes()[9] - b'0');
            }
            other => prop_assert!(false, "expected RUC components, got {other:?}"),
        }
        prop_assert_eq!(components(&value), Ok(validation.components));
    }

    #[test]
    fn public_ruc_round_trip_and_components(value in public_ruc_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "ruc");
        prop_assert_eq!(validate_with(&value, "ruc"), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate(&value), Ok(validation));
        prop_assert!(validate_with(&value, "id_card").is_err());

        match validation.components {
            Some(Components::Ruc(ruc)) => {
                let province = number(&value[..2]);
                prop_assert_eq!(ruc.taxpayer, Taxpayer::PublicEntity);
                prop_assert_eq!(ruc.province.code, province);
                prop_assert_eq!(ruc.province.name.is_none(), province == 50);
                prop_assert_eq!(ruc.id_card, None);
                prop_assert_eq!(ruc.serial, Some(number(&value[3..8])));
            }
            other => prop_assert!(false, "expected RUC components, got {other:?}"),
        }
        prop_assert_eq!(components(&value), Ok(validation.components));
    }

    #[test]
    fn private_ruc_round_trip_and_components(value in private_ruc_value()) {
        // Arbitrary final sequential digits pass: this branch has no checksum.
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "ruc");
        prop_assert_eq!(validate_with(&value, "ruc"), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate(&value), Ok(validation));
        prop_assert!(validate_with(&value, "id_card").is_err());

        match validation.components {
            Some(Components::Ruc(ruc)) => {
                let province = number(&value[..2]);
                prop_assert_eq!(ruc.taxpayer, Taxpayer::PrivateCompany);
                prop_assert_eq!(ruc.province.code, province);
                prop_assert_eq!(ruc.province.name.is_none(), province == 50);
                prop_assert_eq!(ruc.id_card, None);
                prop_assert_eq!(ruc.serial, None);
            }
            other => prop_assert!(false, "expected RUC components, got {other:?}"),
        }
        prop_assert_eq!(components(&value), Ok(validation.components));
    }

    #[test]
    fn changing_id_card_check_digit_is_rejected(
        value in id_card_value(),
        replacement in 0u8..=9,
    ) {
        let expected = id_check_digit(&value[..9]);
        prop_assume!(replacement != expected);
        let invalid = replace_digit(&value, 9, replacement);
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate(&invalid), mismatch);
        prop_assert_eq!(validate_with(&invalid, "id_card"), mismatch);
        prop_assert_eq!(validate_with(&invalid, "identity_card"), mismatch);
    }

    #[test]
    fn changing_natural_ruc_check_digit_is_rejected(
        value in natural_ruc_value(),
        replacement in 0u8..=9,
    ) {
        let expected = id_check_digit(&value[..9]);
        prop_assume!(replacement != expected);
        let invalid = replace_digit(&value, 9, replacement);
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate(&invalid), mismatch);
        prop_assert_eq!(validate_with(&invalid, "ruc"), mismatch);
    }

    #[test]
    fn changing_public_ruc_check_digit_is_rejected(
        value in public_ruc_value(),
        replacement in 0u8..=9,
    ) {
        let expected = value.as_bytes()[8] - b'0';
        prop_assume!(replacement != expected);
        let invalid = replace_digit(&value, 8, replacement);
        let mismatch = Err(ValidationError::CheckDigitMismatch {
            expected: u32::from(expected),
            received: u32::from(replacement),
        });
        prop_assert_eq!(validate(&invalid), mismatch);
        prop_assert_eq!(validate_with(&invalid, "ruc"), mismatch);
    }

    #[test]
    fn unrepresentable_public_check_digit_is_rejected(
        (province, serial, received) in (ruc_province(), 0u32..=99_999, 0u8..=9)
            .prop_filter("body must require check digit 10", |(province, serial, _)| {
                public_check_digit(&format!("{province:02}6{serial:05}")).is_none()
            }),
    ) {
        let value = format!("{province:02}6{serial:05}{received}0001");
        prop_assert_eq!(validate(&value), Err(ValidationError::UnrepresentableCheckDigit));
        prop_assert_eq!(validate_with(&value, "ruc"), Err(ValidationError::UnrepresentableCheckDigit));
    }

    #[test]
    fn natural_and_private_ruc_suffix_is_001(
        value in prop_oneof![natural_ruc_value(), private_ruc_value()],
        position in 10usize..=12,
        replacement in 0u8..=9,
    ) {
        prop_assume!(replacement != value.as_bytes()[position] - b'0');
        let invalid = replace_digit(&value, position, replacement);
        prop_assert_eq!(validate(&invalid), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid, "ruc"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn public_ruc_suffix_is_0001(
        value in public_ruc_value(),
        position in 9usize..=12,
        replacement in 0u8..=9,
    ) {
        prop_assume!(replacement != value.as_bytes()[position] - b'0');
        let invalid = replace_digit(&value, position, replacement);
        prop_assert_eq!(validate(&invalid), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid, "ruc"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn invalid_regions_are_rejected(
        value in id_card_value(),
        region in 0u8..=99,
    ) {
        prop_assume!(!ID_PROVINCES.contains(&region));
        let invalid = format!("{region:02}{}", &value[2..]);
        let error = Err(ValidationError::InvalidRegion {
            region: u32::from(region),
        });
        prop_assert_eq!(validate(&invalid), error);
        prop_assert_eq!(validate_with(&invalid, "id_card"), error);
    }

    #[test]
    fn legacy_region_does_not_make_a_natural_ruc_valid(value in natural_ruc_value()) {
        let invalid = format!("50{}", &value[2..]);
        prop_assert_eq!(validate(&invalid), Err(ValidationError::InvalidRegion { region: 50 }));
        prop_assert_eq!(validate_with(&invalid, "ruc"), Err(ValidationError::InvalidRegion { region: 50 }));
    }

    #[test]
    fn invalid_taxpayer_types_are_rejected(
        value in natural_ruc_value(),
        person_type in 6u8..=9,
        ruc_type in 7u8..=8,
    ) {
        let invalid_id = replace_digit(&value[..10], 2, person_type);
        prop_assert_eq!(validate(&invalid_id), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid_id, "id_card"), Err(ValidationError::InvalidFormat));

        let invalid_ruc = replace_digit(&value, 2, ruc_type);
        prop_assert_eq!(validate(&invalid_ruc), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(validate_with(&invalid_ruc, "ruc"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn final_hyphen_and_surrounding_whitespace_are_ignored(
        value in prop_oneof![id_card_value(), numeric_ruc_value()],
    ) {
        let kind = if value.len() == 10 { "id_card" } else { "ruc" };
        let formatted = format!("{}-{}", &value[..value.len() - 1], &value[value.len() - 1..]);
        prop_assert_eq!(validate(&formatted), validate(&value));
        prop_assert_eq!(validate_with(&formatted, kind), validate_with(&value, kind));

        let padded = format!(" \t{formatted}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
        prop_assert_eq!(validate_with(&padded, kind), validate_with(&value, kind));
    }

    #[test]
    fn misplaced_hyphens_are_rejected(
        value in prop_oneof![id_card_value(), numeric_ruc_value()],
        position in 0usize..=13,
    ) {
        prop_assume!(position <= value.len() && position != value.len() - 1);
        let kind = if value.len() == 10 { "id_card" } else { "ruc" };
        let mut invalid = value.clone();
        invalid.insert(position, '-');
        prop_assert_eq!(
            validate(&invalid),
            Err(ValidationError::InvalidLength {
                expected: 6,
                found: value.len() + 1,
            })
        );
        prop_assert_eq!(validate_with(&invalid, kind), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn passport_round_trip(value in passport_value()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "passport");
        prop_assert_eq!(validation.components, None);
        prop_assert_eq!(validate_with(&value, "passport"), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate(&value), Ok(validation));
        prop_assert_eq!(IdenumbraECU::validate_with(&value, "passport"), Ok(validation));
        prop_assert_eq!(components(&value), Ok(None));
        prop_assert!(validate_with(&value, "id_card").is_err());
        prop_assert!(validate_with(&value, "ruc").is_err());
    }

    #[test]
    fn explicit_passport_accepts_numeric_values_but_inference_does_not(
        digits in prop::collection::vec(0u8..=9, 6..=9),
    ) {
        let value = digits_to_string(&digits);
        let validation = validate_with(&value, "passport");
        prop_assert_eq!(validation.as_ref().map(|v| v.kind), Ok("passport"));
        prop_assert_eq!(validation.map(|v| v.components), Ok(None));
        prop_assert_eq!(
            validate(&value),
            Err(ValidationError::InvalidLength {
                expected: 10,
                found: value.len(),
            })
        );
    }

    #[test]
    fn passport_non_alphanumerics_are_rejected(
        value in passport_value(),
        position in 0usize..6,
        character in prop::sample::select(&NON_ALNUM[..]),
    ) {
        let at = position % value.len();
        let mut bytes = value.into_bytes();
        bytes[at] = character;
        let invalid = String::from_utf8(bytes).unwrap_or_else(|_| unreachable!("ASCII remains UTF-8"));
        prop_assert_eq!(validate(&invalid), Err(ValidationError::NonAlphanumeric { index: at }));
        prop_assert_eq!(validate_with(&invalid, "passport"), Err(ValidationError::InvalidFormat));
    }

    #[test]
    fn surrounding_whitespace_is_ignored_for_every_kind(value in any_valid_value()) {
        let kind = match value.len() {
            10 => "id_card",
            13 => "ruc",
            _ => "passport",
        };
        let padded = format!("  \t{value}\n ");
        prop_assert_eq!(validate(&padded), validate(&value));
        prop_assert_eq!(validate_with(&padded, kind), validate_with(&value, kind));
    }

    #[test]
    fn digit_strings_of_other_lengths_are_rejected(
        digits in prop::collection::vec(0u8..=9, 0..=20),
    ) {
        prop_assume!(!matches!(digits.len(), 10 | 13));
        let value = digits_to_string(&digits);
        match value.len() {
            0 => prop_assert_eq!(validate(&value), Err(ValidationError::Empty)),
            14.. => prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength { expected: 13, found: 14 })
            ),
            found => prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength { expected: 10, found })
            ),
        }
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(IdenumbraECU::validate(&value), first.clone());
        prop_assert_eq!(components(&value), first.map(|validation| validation.components));
        let _ = validate_with(&value, "id_card");
        let _ = validate_with(&value, "identity_card");
        let _ = validate_with(&value, "ruc");
        let _ = validate_with(&value, "passport");
        let _ = validate_with(&value, "unknown");
        let _ = IdenumbraECU::validate_with(&value, "id_card");
        let _ = IdenumbraECU::validate_with(&value, "ruc");
        let _ = IdenumbraECU::validate_with(&value, "passport");
    }

    #[test]
    fn unknown_kinds_are_rejected(
        value in any_valid_value(),
        kind in prop::sample::select(&["", "ID_CARD", "ruc ", "ci", "nit"][..]),
    ) {
        prop_assert_eq!(validate_with(&value, kind), Err(ValidationError::InvalidFormat));
        prop_assert_eq!(IdenumbraECU::validate_with(&value, kind), Err(ValidationError::InvalidFormat));
    }
}
