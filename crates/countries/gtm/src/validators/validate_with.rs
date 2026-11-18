/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        "cui" | "dpi" => {
            if value.bytes().all(|b| b.is_ascii_digit()) {
                validate_cui(value.as_bytes())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "nit" => {
            // Since 2026 the DPI/CUI number is also used as NIT.
            if value.len() == 13 && value.bytes().all(|b| b.is_ascii_digit()) {
                return validate_cui(value.as_bytes());
            }
            if value.bytes().all(|b| b.is_ascii_alphanumeric()) {
                validate_nit(value.as_bytes())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
