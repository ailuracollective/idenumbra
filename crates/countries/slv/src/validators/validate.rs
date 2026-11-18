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

fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 9,
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

/// Validate a Salvadoran identification number (DUI or NIT).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().any(|b| b.is_ascii_alphabetic()) {
        return validate_passport(value);
    }
    let mut buf = [0u8; 14];
    let n = compact_digits(value, &mut buf)?;
    match n {
        9 => validate_dui(&buf[..9]),
        14 => validate_nit(&buf[..14]),
        _ => Err(ValidationError::InvalidLength {
            expected: 9,
            found: n,
        }),
    }
}
