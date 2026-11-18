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

/// Validate an Argentine identification number (DNI or CUIT/CUIL).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 11];
    let n = compact_digits(value, &mut buf)?;
    match n {
        7 | 8 => {
            if buf[..n].iter().all(|d| *d == b'0') {
                return Err(ValidationError::InvalidFormat);
            }
            Ok(()) // DNI
        }
        11 => validate_cuit(&buf[..11]),
        _ => Err(ValidationError::InvalidLength {
            expected: 8,
            found: n,
        }),
    }
}
