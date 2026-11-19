//! Property-based tests for the Brazilian CPF and CNPJ validators.

use idenumbra_bra::{components, validate, validate_with, Components, IdenumbraBRA, Validation};
use idenumbra_core::{ValidationError, Validator};
use proptest::prelude::*;

fn cpf_dv(first9: &[u8]) -> (u8, u8) {
    let mut sum = 0u32;
    for (i, &b) in first9.iter().enumerate() {
        sum += (b - b'0') as u32 * (10 - i as u32);
    }
    let rest = sum % 11;
    let dv1 = if rest < 2 { 0 } else { 11 - rest };
    let mut sum = 0u32;
    for (i, &b) in first9.iter().enumerate() {
        sum += (b - b'0') as u32 * (11 - i as u32);
    }
    sum += dv1 * 2;
    let rest = sum % 11;
    let dv2 = if rest < 2 { 0 } else { 11 - rest };
    (dv1 as u8, dv2 as u8)
}

fn cnpj_dv(first12: &[u8]) -> (u8, u8) {
    let w1 = [5u32, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;
    for (i, &b) in first12.iter().enumerate() {
        sum += (b as u32 - 48) * w1[i];
    }
    let rest = sum % 11;
    let dv1 = if rest < 2 { 0 } else { 11 - rest };
    let w2 = [6u32, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;
    for (i, &b) in first12.iter().enumerate() {
        sum += (b as u32 - 48) * w2[i];
    }
    sum += dv1 * w2[12];
    let rest = sum % 11;
    let dv2 = if rest < 2 { 0 } else { 11 - rest };
    (dv1 as u8, dv2 as u8)
}

fn accepted(value: &str) -> Validation {
    match validate(value) {
        Ok(v) => v,
        Err(e) => panic!("expected {value:?} to be valid, got {e:?}"),
    }
}

fn cpf_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 9)
        .prop_map(|digits| {
            let first: Vec<u8> = digits.iter().map(|d| b'0' + d).collect();
            let (dv1, dv2) = cpf_dv(&first);
            let mut s: String = first.iter().map(|&b| b as char).collect();
            s.push(char::from(b'0' + dv1));
            s.push(char::from(b'0' + dv2));
            s
        })
        .prop_filter("not an all-same placeholder", |s| {
            let b = s.as_bytes();
            !b.iter().all(|x| *x == b[0])
        })
}

fn numeric_cnpj_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(0u8..=9, 12)
        .prop_map(|digits| {
            let first: Vec<u8> = digits.iter().map(|d| b'0' + d).collect();
            let (dv1, dv2) = cnpj_dv(&first);
            let mut s: String = first.iter().map(|&b| b as char).collect();
            s.push(char::from(b'0' + dv1));
            s.push(char::from(b'0' + dv2));
            s
        })
        .prop_filter("not an all-same placeholder", |s| {
            let b = s.as_bytes();
            !b.iter().all(|x| *x == b[0])
        })
}

const ALNUM: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn alnum_cnpj_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(ALNUM), 12)
        .prop_map(|chars| {
            let first: Vec<u8> = chars;
            let (dv1, dv2) = cnpj_dv(&first);
            let mut s: String = first.iter().map(|&b| b as char).collect();
            s.push(char::from(b'0' + dv1));
            s.push(char::from(b'0' + dv2));
            s
        })
        .prop_filter("not an all-same placeholder", |s| {
            let b = s.as_bytes();
            !b.iter().all(|x| *x == b[0])
        })
}

proptest! {
    #[test]
    fn cpf_round_trip(value in cpf_strategy()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "cpf");
        prop_assert!(validation.components.is_some());
        prop_assert!(validate_with(&value, "cpf").is_ok());
        prop_assert!(validate_with(&value, "cnpj").is_err());
        prop_assert_eq!(IdenumbraBRA::validate(&value), Ok(validation));
        prop_assert_eq!(validate(&value), validate_with(&value, "cpf"));
    }

    #[test]
    fn cpf_formatting_accepted(value in cpf_strategy()) {
        let f = format!("{}.{}.{}-{}", &value[0..3], &value[3..6], &value[6..9], &value[9..11]);
        prop_assert!(validate(&f).is_ok());
        prop_assert_eq!(validate(&f), validate(&value));
    }

    #[test]
    fn cpf_components(value in cpf_strategy()) {
        let v = accepted(&value);
        match v.components {
            Some(Components::Cpf(c)) => {
                prop_assert_eq!(c.region, value.as_bytes()[8] - b'0');
                prop_assert_eq!(c.check_digit_1, value.as_bytes()[9] - b'0');
                prop_assert_eq!(c.check_digit_2, value.as_bytes()[10] - b'0');
            }
            _ => prop_assert!(false, "expected cpf components"),
        }
        prop_assert_eq!(components(&value), Ok(v.components));
    }

    #[test]
    fn cpf_wrong_first_check_digit(value in cpf_strategy()) {
        let mut bytes = value.as_bytes().to_vec();
        let orig = bytes[9];
        let wrong = if orig == b'0' { b'1' } else { b'0' };
        bytes[9] = wrong;
        let invalid = String::from_utf8(bytes).unwrap();
        let res = validate(&invalid);
        let ok = matches!(
            res,
            Err(ValidationError::CheckDigitMismatch { expected, received })
                if expected == (orig - b'0') as u32 && received == (wrong - b'0') as u32
        );
        prop_assert!(ok);
    }

    #[test]
    fn cpf_wrong_second_check_digit(value in cpf_strategy()) {
        let mut bytes = value.as_bytes().to_vec();
        let orig = bytes[10];
        let wrong = if orig == b'0' { b'1' } else { b'0' };
        bytes[10] = wrong;
        let invalid = String::from_utf8(bytes).unwrap();
        let res = validate(&invalid);
        let ok = matches!(
            res,
            Err(ValidationError::CheckDigitMismatch { expected, received })
                if expected == (orig - b'0') as u32 && received == (wrong - b'0') as u32
        );
        prop_assert!(ok);
    }

    #[test]
    fn numeric_cnpj_round_trip(value in numeric_cnpj_strategy()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "cnpj");
        prop_assert!(validation.components.is_some());
        prop_assert!(validate_with(&value, "cnpj").is_ok());
        prop_assert!(validate_with(&value, "cpf").is_err());
        prop_assert_eq!(IdenumbraBRA::validate(&value), Ok(validation));
        prop_assert_eq!(validate(&value), validate_with(&value, "cnpj"));
    }

    #[test]
    fn numeric_cnpj_formatting_accepted(value in numeric_cnpj_strategy()) {
        let simple = format!("{}.{}.{}/{}-{}", &value[0..2], &value[2..5], &value[5..8], &value[8..12], &value[12..14]);
        prop_assert!(validate(&simple).is_ok());
        prop_assert_eq!(validate(&simple), validate(&value));
    }

    #[test]
    fn numeric_cnpj_components(value in numeric_cnpj_strategy()) {
        let v = accepted(&value);
        match v.components {
            Some(Components::Cnpj(c)) => {
                let mut root = [0u8; 8];
                root.copy_from_slice(&value.as_bytes()[..8]);
                prop_assert_eq!(c.root, root);
                let branch: u32 = value[8..12].bytes().fold(0u32, |a, b| a * 10 + (b - b'0') as u32);
                prop_assert_eq!(c.branch, branch);
                prop_assert_eq!(c.check_digit_1, value.as_bytes()[12] - b'0');
                prop_assert_eq!(c.check_digit_2, value.as_bytes()[13] - b'0');
            }
            _ => prop_assert!(false, "expected cnpj components"),
        }
        prop_assert_eq!(components(&value), Ok(v.components));
    }

    #[test]
    fn numeric_cnpj_wrong_first_check(value in numeric_cnpj_strategy()) {
        let mut bytes = value.as_bytes().to_vec();
        let orig = bytes[12];
        let wrong = if orig == b'0' { b'1' } else { b'0' };
        bytes[12] = wrong;
        let invalid = String::from_utf8(bytes).unwrap();
        let res = validate(&invalid);
        let ok = matches!(
            res,
            Err(ValidationError::CheckDigitMismatch { expected, received })
                if expected == (orig - b'0') as u32 && received == (wrong - b'0') as u32
        );
        prop_assert!(ok);
    }

    #[test]
    fn alnum_cnpj_round_trip(value in alnum_cnpj_strategy()) {
        let validation = accepted(&value);
        prop_assert_eq!(validation.kind, "cnpj");
        prop_assert!(validation.components.is_some());
        prop_assert!(validate_with(&value, "cnpj").is_ok());
        let lower = value.to_lowercase();
        prop_assert!(validate(&lower).is_ok());
        prop_assert_eq!(validate(&lower), validate(&value));
    }

    #[test]
    fn alnum_cnpj_components(value in alnum_cnpj_strategy()) {
        let v = accepted(&value);
        match v.components {
            Some(Components::Cnpj(c)) => {
                let mut root = [0u8; 8];
                root.copy_from_slice(&value.as_bytes()[..8]);
                prop_assert_eq!(c.root, root);
                prop_assert_eq!(c.check_digit_1, value.as_bytes()[12] - b'0');
                prop_assert_eq!(c.check_digit_2, value.as_bytes()[13] - b'0');
            }
            _ => prop_assert!(false, "expected cnpj components"),
        }
    }

    #[test]
    fn alnum_cnpj_wrong_check_digits(value in alnum_cnpj_strategy()) {
        let mut bytes = value.as_bytes().to_vec();
        let orig = bytes[12];
        let wrong = if orig == b'0' { b'1' } else { b'0' };
        bytes[12] = wrong;
        let invalid = String::from_utf8(bytes).unwrap();
        let res = validate(&invalid);
        let ok = matches!(
            res,
            Err(ValidationError::CheckDigitMismatch { expected, received })
                if expected == (orig - b'0') as u32 && received == (wrong - b'0') as u32
        );
        prop_assert!(ok);
    }

    #[test]
    fn lengths_rejected(n in 0usize..=20) {
        prop_assume!(n != 11 && n != 14);
        let value: String = "1".repeat(n);
        if value.is_empty() {
            prop_assert_eq!(validate(&value), Err(ValidationError::Empty));
        } else if n > 14 {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength { expected: 14, found: 15 })
            );
        } else {
            prop_assert_eq!(
                validate(&value),
                Err(ValidationError::InvalidLength { expected: 11, found: n })
            );
        }
    }

    #[test]
    fn arbitrary_strings_never_panic(value in any::<String>()) {
        let first = validate(&value);
        prop_assert_eq!(first.clone(), validate(&value));
        let _ = validate_with(&value, "cpf");
        let _ = validate_with(&value, "cnpj");
        let _ = validate_with(&value, "passport");
        let _ = IdenumbraBRA::validate(&value);
        let _ = components(&value);
    }

    #[test]
    fn surrounding_whitespace_ignored(value in cpf_strategy()) {
        let padded = format!("  \t{}\n ", value);
        prop_assert_eq!(validate(&padded), validate(&value));
    }
}

#[test]
fn repeated_placeholders_rejected() {
    for d in 0u8..=9 {
        let arr = [b'0' + d; 11];
        let s = std::str::from_utf8(&arr).unwrap();
        assert_eq!(validate(s), Err(ValidationError::InvalidFormat));
    }
    for &c in ALNUM {
        let arr = [c; 14];
        let s = std::str::from_utf8(&arr).unwrap();
        assert_eq!(
            validate(s),
            Err(ValidationError::InvalidFormat),
            "cnpj all {}",
            c as char
        );
    }
}

#[test]
fn invalid_chars_rejected() {
    assert!(validate("12.ABÇ.345/01DE-35").is_err());
    assert!(validate("12.ABÑ.345/01DE-35").is_err());
    assert!(validate("12.ABC.345/01DE-E5").is_err());
    assert!(validate("1234567890A").is_err());
}

#[test]
fn explicit_kind_mismatch() {
    assert!(validate_with("123.456.789-09", "cpf").is_ok());
    assert!(validate_with("123.456.789-09", "cnpj").is_err());
    assert!(validate_with("12.345.678/0001-95", "cnpj").is_ok());
    assert!(validate_with("12.345.678/0001-95", "cpf").is_err());
}

#[test]
fn unknown_kinds_rejected_concrete() {
    for kind in ["", "passport", "CPF", "cpf ", "nit"] {
        assert!(validate_with("12345678909", kind).is_err(), "{kind}");
    }
}
