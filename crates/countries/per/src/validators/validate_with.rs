/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            if value.bytes().all(|b| b.is_ascii_digit()) && value.len() == 8 {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "ruc" => {
            if value.bytes().all(|b| b.is_ascii_digit()) {
                validate_ruc(value.as_bytes())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" | "ce" => {
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
