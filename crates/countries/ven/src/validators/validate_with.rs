/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            if value.bytes().all(|b| b.is_ascii_digit()) && (6..=8).contains(&value.len()) {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "rif" => {
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
            validate_rif(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
