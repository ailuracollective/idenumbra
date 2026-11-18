/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" | "identity_card" => {
            let mut buf = [0u8; MAX_DIGITS];
            match digits(value, &mut buf) {
                Ok(10) => {
                    validate_id_card(&buf[..10])?;
                    Ok(Validation {
                        kind: "id_card",
                        components: Some(Components::IdCard(id_card(&buf[..10]))),
                    })
                }
                Ok(found) => Err(ValidationError::InvalidLength {
                    expected: 10,
                    found,
                }),
                Err(_) => Err(ValidationError::InvalidFormat),
            }
        }
        "ruc" => {
            let mut buf = [0u8; MAX_DIGITS];
            match digits(value, &mut buf) {
                Ok(13) => {
                    validate_ruc(&buf[..13])?;
                    Ok(Validation {
                        kind: "ruc",
                        components: Some(Components::Ruc(ruc(&buf[..13]))),
                    })
                }
                Ok(found) => Err(ValidationError::InvalidLength {
                    expected: 13,
                    found,
                }),
                Err(_) => Err(ValidationError::InvalidFormat),
            }
        }
        "passport" => {
            if value.bytes().all(|b| b.is_ascii_alphanumeric()) {
                validate_passport(value)?;
                Ok(Validation {
                    kind: "passport",
                    components: None,
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}