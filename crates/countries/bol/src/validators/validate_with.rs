/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            let (digits_len, rest) = split_complement(value);
            if (5..=8).contains(&digits_len)
                && (rest.is_empty() || valid_complement(rest))
                && !all_zero(&value.as_bytes()[..digits_len])
            {
                Ok(Validation {
                    kind: "id_card",
                    components: None,
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "nit" => {
            let (digits_len, rest) = split_complement(value);
            if !rest.is_empty() {
                return Err(ValidationError::InvalidFormat);
            }
            validate_nit(&value.as_bytes()[..digits_len])?;
            let bytes = &value.as_bytes()[..digits_len];
            Ok(Validation {
                kind: "nit",
                components: Some(Components::Nit(nit(bytes))),
            })
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}