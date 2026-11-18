/// Clean a RUT into `buf`, skipping dots, hyphens and spaces.
fn compact(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: buf.len(),
                    found: n + 1,
                });
            }
            buf[n] = b.to_ascii_uppercase();
            n += 1;
        } else if !matches!(b, b'.' | b'-' | b' ') {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(n)
}

/// Validate a Chilean RUT/RUN.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 9];
    let n = compact(value, &mut buf)?;
    if n < 2 {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: n,
        });
    }
    validate_rut(&buf[..n])
}
