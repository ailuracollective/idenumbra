/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            // All-zero placeholders are never issued; `validate` rejects them.
            if value.bytes().all(|b| b.is_ascii_digit())
                && value.len() == 8
                && value.bytes().any(|b| b != b'0')
            {
                Ok(Validation {
                    kind: "dni",
                    components: None,
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "ruc" => {
            if value.bytes().all(|b| b.is_ascii_digit()) {
                validate_ruc(value.as_bytes())?;
                Ok(Validation {
                    kind: "ruc",
                    components: Some(Components::Ruc(ruc(value.as_bytes()))),
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" => {
            if (6..=12).contains(&value.len())
                && value.bytes().all(|b| b.is_ascii_alphanumeric())
            {
                Ok(Validation {
                    kind: "passport",
                    components: None,
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "ce" => {
            if (6..=12).contains(&value.len())
                && value.bytes().all(|b| b.is_ascii_alphanumeric())
            {
                Ok(Validation {
                    kind: "ce",
                    components: None,
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}