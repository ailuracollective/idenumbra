/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    let mut buf = [0u8; 14];
    match kind {
        "cpf" => {
            let n = compact_chars(value, &mut buf[..11])?;
            if n != 11 {
                return Err(ValidationError::InvalidLength {
                    expected: 11,
                    found: n,
                });
            }
            validate_cpf(&buf[..11])?;
            Ok(Validation {
                kind: "cpf",
                components: Some(Components::Cpf(cpf(&buf[..11]))),
            })
        }
        "cnpj" => {
            let n = compact_chars(value, &mut buf)?;
            if n != 14 {
                return Err(ValidationError::InvalidLength {
                    expected: 14,
                    found: n,
                });
            }
            validate_cnpj(&buf[..14])?;
            Ok(Validation {
                kind: "cnpj",
                components: Some(Components::Cnpj(cnpj(&buf[..14]))),
            })
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}