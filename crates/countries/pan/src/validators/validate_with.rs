/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    match kind {
        "id_card" | "cedula" | "identity_card" => {
            validate_id_card(value)?;
            let components = if value.contains('-') {
                parse_cedula(value).map(Components::Cedula)
            } else {
                None
            };
            Ok(Validation {
                kind: "id_card",
                components,
            })
        }
        "ruc" => {
            validate_ruc(value)?;
            let components = if value.contains('-') {
                parse_cedula(value).map(Components::Cedula)
            } else {
                None
            };
            Ok(Validation {
                kind: "ruc",
                components,
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