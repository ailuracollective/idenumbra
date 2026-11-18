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
            // Since 2023-10-04 a natural person's NIT is built from the CUI, and
            // SAT accepts the full 13-digit CUI (Resolución SAT-DSI-393-2025).
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
