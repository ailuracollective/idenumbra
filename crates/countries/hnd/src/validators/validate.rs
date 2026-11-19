/// Copy digits into `buf`, skipping hyphens and spaces.
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
        } else if !matches!(b, b'-' | b' ') {
            return Err(ValidationError::NonNumeric { index: i });
        }
    }
    Ok(n)
}

fn all_zero(digits: &[u8]) -> bool {
    !digits.is_empty() && digits.iter().all(|d| *d == b'0')
}

fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 6,
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

/// Validate a Honduran identification number (DNI or RTN).
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().any(|b| b.is_ascii_alphabetic()) {
        validate_passport(value)?;
        return Ok(Validation {
            kind: "passport",
            components: None,
        });
    }
    let mut buf = [0u8; 14];
    let n = compact_digits(value, &mut buf)?;
    match n {
        13 => {
            validate_dni(&buf[..13])?;
            Ok(Validation {
                kind: "dni",
                components: None,
            })
        }
        14 => {
            validate_rtn(&buf[..14])?;
            Ok(Validation {
                kind: "rtn",
                components: Some(Components::Rtn(rtn_from_digits(&buf[..14]))),
            })
        }
        _ => Err(ValidationError::InvalidLength {
            expected: 13,
            found: n,
        }),
    }
}
