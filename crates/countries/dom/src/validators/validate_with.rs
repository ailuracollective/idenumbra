/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            // `validate` strips the hyphens and spaces people type; reuse it and
            // reject the values it would read as a passport.
            if value
                .bytes()
                .all(|b| b.is_ascii_digit() || matches!(b, b'-' | b' '))
            {
                validate(value)
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
