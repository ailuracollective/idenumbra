/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "curp" => {
            let mut buf = [0u8; 18];
            let n = compact(value, &mut buf)?;
            if n != 18 {
                return Err(ValidationError::InvalidLength {
                    expected: 18,
                    found: n,
                });
            }
            validate_curp(&buf[..18])
        }
        "rfc" => {
            let mut buf = [0u8; 18];
            let n = compact(value, &mut buf)?;
            validate_rfc(&buf[..n])
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}
