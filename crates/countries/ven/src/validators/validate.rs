/// RIF prefixes and the base value each adds to the checksum: V venezolano,
/// E extranjero, J jurídico, P pasaporte, G gobierno, C comuna (same base as
/// J: SENIAT migrated the comunas that already had a J RIF in 2015 keeping
/// their digits, so C cannot differ; the `C = 24` variant is wrong). SENIAT
/// publishes no letter table or formula; edit when a new letter appears.
const RIF_PREFIXES: [(u8, u32); 6] = [
    (b'V', 4),
    (b'E', 8),
    (b'J', 12),
    (b'P', 16),
    (b'G', 20),
    (b'C', 12),
];

fn rif_base(letter: u8) -> Option<u32> {
    RIF_PREFIXES
        .iter()
        .find(|(l, _)| *l == letter)
        .map(|(_, base)| *base)
}

/// Validate a Venezuelan RIF (letter + 8 digits + check digit).
fn validate_rif(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 10 {
        return Err(ValidationError::InvalidLength {
            expected: 10,
            found: chars.len(),
        });
    }
    let Some(base) = rif_base(chars[0]) else {
        return Err(ValidationError::InvalidFormat);
    };
    if !chars[1..].iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 1 });
    }
    if chars[1..9].iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
    let mut sum = base;
    for (d, w) in chars[1..9].iter().zip(weights) {
        sum += (d - b'0') as u32 * w;
    }
    let rest = sum % 11;
    // 11 - remainder, with 10 and 11 represented as 0.
    let computed = if rest < 2 { 0 } else { 11 - rest };
    let received = (chars[9] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}

fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: value.len(),
        });
    }
    for (i, b) in value.bytes().enumerate() {
        if !b.is_ascii_alphanumeric() {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(())
}

/// Validate a Venezuelan identification number (CI or RIF).
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 12];
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: 10,
                    found: n + 1,
                });
            }
            buf[n] = b.to_ascii_uppercase();
            n += 1;
        } else if !matches!(b, b'-' | b' ') {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    if n == 10
        && rif_base(buf[0]).is_some()
        && buf[1..10].iter().all(u8::is_ascii_digit)
    {
        validate_rif(&buf[..10])?;
        return Ok(Validation {
            kind: "rif",
            components: Some(Components::Rif(rif(&buf[..10]))),
        });
    }
    if buf[..n].iter().all(u8::is_ascii_digit) {
        if buf[..n].iter().all(|d| *d == b'0') {
            return Err(ValidationError::InvalidFormat);
        }
        return if (6..=8).contains(&n) {
            Ok(Validation {
                kind: "ci",
                components: None,
            })
        } else {
            Err(ValidationError::InvalidLength {
                expected: 8,
                found: n,
            })
        };
    }
    validate_passport(value)?;
    Ok(Validation {
        kind: "passport",
        components: None,
    })
}
