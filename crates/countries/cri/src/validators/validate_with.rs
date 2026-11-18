/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            if value.bytes().all(|b| b.is_ascii_digit()) && value.len() == 9 {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "dimex" => {
            if value.bytes().all(|b| b.is_ascii_digit()) && DIMEX_LENGTHS.contains(&value.len()) {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" => {
            if value.len() >= 6
                && value.len() <= 12
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
