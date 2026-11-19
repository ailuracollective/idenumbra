/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "dni" => {
            let mut buf = [0u8; 11];
            let n = compact_digits(value, &mut buf)?;
            validate_dni(&buf[..n])?;
            Ok(Validation {
                kind: "dni",
                components: None,
            })
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
            validate_cuit(&buf[..11])?;
            let reported_kind = if kind == "cuil" { "cuil" } else { "cuit" };
            Ok(Validation {
                kind: reported_kind,
                components: Some(Components::Cuit(cuit(&buf[..11]))),
            })
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}