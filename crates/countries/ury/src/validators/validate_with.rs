/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "ci" | "id_card" | "identity_card" => {
            let mut buf = [0u8; 12];
            let n = compact_digits(value, &mut buf)?;
            validate_ci(&buf[..n])?;
            Ok(Validation {
                kind: "ci",
                components: Some(Components::Ci(ci(&buf[..n]))),
            })
        }
        "rut" => {
            let mut buf = [0u8; 12];
            let n = compact_digits(value, &mut buf)?;
            validate_rut(&buf[..n])?;
            Ok(Validation {
                kind: "rut",
                components: Some(Components::Rut(rut(&buf[..n]))),
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