/// Copy alphanumeric characters into `buf`, skipping the usual separators.
fn compact_chars(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
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
        } else if !matches!(b, b'.' | b'-' | b'/' | b' ') {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(n)
}

/// Validate a Brazilian identification number (CPF or CNPJ).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 14];
    let n = compact_chars(value, &mut buf)?;
    match n {
        11 => validate_cpf(&buf[..11]),
        14 => validate_cnpj(&buf[..14]),
        _ => Err(ValidationError::InvalidLength {
            expected: 11,
            found: n,
        }),
    }
}
