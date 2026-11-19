/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    let mut buf = [0u8; 14];
    let n = depurate(value, &mut buf)?;
    let bytes = &buf[..n];
    match kind {
        // A CUI has 12 or 13 digits (SAT §5.1.4.1); the 12-digit form is a
        // correlativo that lost its leading zero. §5.1.2 depuration is applied
        // first, so the printed "AAAA BBBBB CCCC" form is accepted.
        "cui" | "dpi" => {
            if bytes.iter().all(u8::is_ascii_digit) {
                validate_cui(bytes)?;
                Ok(Validation {
                    kind: "cui",
                    components: Some(Components::Cui(cui(bytes))),
                })
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        // Since 2023-10-04 a natural person's NIT is the first 9 digits of the
        // CUI, and Resolución SAT-DSI-393-2025 also accepts the full 13-digit
        // CUI, which is therefore checked as a CUI.
        "nit" => {
            if n == 13 && bytes.iter().all(u8::is_ascii_digit) {
                validate_cui(bytes)?;
                return Ok(Validation {
                    kind: "nit",
                    components: Some(Components::Cui(cui(bytes))),
                });
            }
            validate_nit(bytes)?;
            Ok(Validation {
                kind: "nit",
                components: Some(Components::Nit(nit(bytes))),
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