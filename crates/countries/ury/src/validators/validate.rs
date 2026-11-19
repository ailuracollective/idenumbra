/// Copy digits into `buf`, skipping dots, hyphens and spaces.
fn compact_digits(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_digit() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: buf.len(),
                    found: n + 1,
                });
            }
            buf[n] = b;
            n += 1;
        } else if !matches!(b, b'.' | b'-' | b' ') {
            return Err(ValidationError::NonNumeric { index: i });
        }
    }
    Ok(n)
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

/// Validate a Uruguayan cédula de identidad (6-7 digit body plus check digit).
fn validate_ci(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 7 && digits.len() != 8 {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let (body, dv) = digits.split_at(digits.len() - 1);
    // The 6-digit body is left-padded to 7 digits with zeros.
    let mut padded = [b'0'; 7];
    padded[7 - body.len()..].copy_from_slice(body);
    let weights = [2u32, 9, 8, 7, 6, 3, 4];
    let sum: u32 = padded
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    let computed = (10 - sum % 10) % 10;
    let received = (dv[0] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}

/// Validate a Uruguayan identification number (CI or RUT).
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 12];
    let mut n = 0usize;
    for b in value.bytes() {
        if b.is_ascii_digit() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: 12,
                    found: n + 1,
                });
            }
            buf[n] = b;
            n += 1;
        } else if !matches!(b, b'.' | b'-' | b' ') {
            validate_passport(value)?;
            return Ok(Validation {
                kind: "passport",
                components: None,
            });
        }
    }
    match n {
        7 | 8 => {
            validate_ci(&buf[..n])?;
            Ok(Validation {
                kind: "ci",
                components: Some(Components::Ci(ci(&buf[..n]))),
            })
        }
        12 => {
            // validate_rut is defined in rut.rs (same module).
            validate_rut(&buf[..12])?;
            Ok(Validation {
                kind: "rut",
                components: Some(Components::Rut(rut(&buf[..12]))),
            })
        }
        _ => Err(ValidationError::InvalidLength {
            expected: 8,
            found: n,
        }),
    }
}