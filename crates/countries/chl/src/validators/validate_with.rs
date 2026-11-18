/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    match kind {
"rut" | "run" => validate(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
