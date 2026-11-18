/// Copy ASCII digits into `buf`, skipping the usual CUIT separators.
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
        } else if !matches!(b, b'.' | b'-' | b' ' | b'/') {
            return Err(ValidationError::NonNumeric { index: i });
        }
    }
    Ok(n)
}

/// DNI: 7-8 digits, no check digit; all-zero placeholders are never issued.
fn validate_dni(digits: &[u8]) -> Result<(), ValidationError> {
    if !(7..=8).contains(&digits.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: digits.len(),
        });
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Validate an Argentine identification number (DNI or CUIT/CUIL).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 11];
    let n = compact_digits(value, &mut buf)?;
    match n {
        7 | 8 => validate_dni(&buf[..n]),
        11 => validate_cuit(&buf[..11]),
        _ => Err(ValidationError::InvalidLength {
            expected: 8,
            found: n,
        }),
    }
}
