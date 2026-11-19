/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            validate_dni(&buf[..n])?;
            Ok(Validation {
                kind: "dni",
                components: None,
            })
        }
        "rtn" => {
            let mut buf = [0u8; 14];
            let n = compact_digits(value, &mut buf)?;
            validate_rtn(&buf[..n])?;
            Ok(Validation {
                kind: "rtn",
                components: Some(Components::Rtn(rtn_from_digits(&buf[..14]))),
            })
        }
        "passport" => {
            validate_passport(value)?;
            Ok(Validation {
                kind: "passport",
                components: None,
            })
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}