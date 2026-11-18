/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            let mut buf = [0u8; 11];
            let n = compact_digits(value, &mut buf)?;
            if n == 7 || n == 8 {
                Ok(())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "cuit" | "cuil" => {
            let mut buf = [0u8; 11];
            let n = compact_digits(value, &mut buf)?;
            if n != 11 {
                return Err(ValidationError::InvalidLength {
                    expected: 11,
                    found: n,
                });
            }
            validate_cuit(&buf[..11])
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}
