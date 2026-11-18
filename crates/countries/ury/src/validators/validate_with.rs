/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            let mut buf = [0u8; 12];
            let n = compact_digits(value, &mut buf)?;
            validate_ci(&buf[..n])
        }
        "rut" => {
            let mut buf = [0u8; 12];
            let n = compact_digits(value, &mut buf)?;
            validate_rut(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
