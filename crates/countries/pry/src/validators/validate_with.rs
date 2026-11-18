/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" | "ci" => {
            if value.bytes().all(|b| b.is_ascii_digit())
                && (6..=8).contains(&value.len())
                && value.bytes().any(|b| b != b'0')
            {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "ruc" => {
            let mut buf = [0u8; 9];
            let mut n = 0usize;
            for (i, b) in value.bytes().enumerate() {
                if b.is_ascii_digit() {
                    if n >= buf.len() {
                        return Err(ValidationError::InvalidLength {
                            expected: 9,
                            found: n + 1,
                        });
                    }
                    buf[n] = b;
                    n += 1;
                } else if !matches!(b, b'.' | b'-' | b' ') {
                    return Err(ValidationError::NonNumeric { index: i });
                }
            }
            validate_ruc(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
