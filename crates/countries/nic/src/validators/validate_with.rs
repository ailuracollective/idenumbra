/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            let mut buf = [0u8; 14];
            let n = compact(value, &mut buf)?;
            validate_id_card(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
