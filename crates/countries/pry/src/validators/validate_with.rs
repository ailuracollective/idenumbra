/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" | "ci" => {
            let mut buf = [0u8; 9];
            let n = compact_digits(value, &mut buf)?;
            if (6..=8).contains(&n) && buf[..n].iter().any(|d| *d != b'0') {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "ruc" => {
            let mut buf = [0u8; 9];
            let n = compact_digits(value, &mut buf)?;
            validate_ruc(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
