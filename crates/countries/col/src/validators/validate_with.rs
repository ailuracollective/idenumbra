/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "citizen_id" | "id_card" => {
            // All-zero placeholders are never issued; `validate` rejects them.
            if value.bytes().all(|b| b.is_ascii_digit())
                && (6..=10).contains(&value.len())
                && value.bytes().any(|b| b != b'0')
            {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "nit" => {
            let mut buf = [0u8; 12];
            let n = compact_digits(value, &mut buf)?;
            validate_nit(&buf[..n])
        }
        "passport" => {
            if (6..=12).contains(&value.len())
                && value.bytes().all(|b| b.is_ascii_alphanumeric())
            {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}
