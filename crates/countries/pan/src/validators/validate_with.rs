/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
"id_card" | "ruc" => validate(value),
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
