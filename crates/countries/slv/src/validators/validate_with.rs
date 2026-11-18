/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "dui" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            validate_dui(&buf[..n])
        }
        // Since 2021 a DUI is also a valid NIT for natural persons.
        "nit" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            if n == 9 {
                return validate_dui(&buf[..9]);
            }
            validate_nit(&buf[..n])
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
