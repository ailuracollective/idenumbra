/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            validate_dni(&buf[..n])
        }
        "rtn" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            validate_rtn(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
