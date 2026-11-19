/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" => {
            let mut buf = [0u8; 14];
            let n = compact(value, &mut buf)?;
            validate_id_card(&buf[..n])?;
            Ok(Validation {
                kind: "id_card",
                components: Some(Components::Cedula(cedula(&buf[..n]))),
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