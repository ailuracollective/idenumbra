/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    match kind {
        // A CUI has 12 or 13 digits (SAT §5.1.4.1); the 12-digit form is a
        // correlativo that lost its leading zero.
        "cui" | "dpi" => {
            if value.bytes().all(|b| b.is_ascii_digit()) {
                validate_cui(value.as_bytes())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        // Since 2023-10-04 a natural person's NIT is the first 9 digits of the
        // CUI, and Resolución SAT-DSI-393-2025 also accepts the full 13-digit
        // CUI, which is therefore checked as a CUI.
        "nit" => {
            if value.len() == 13 && value.bytes().all(|b| b.is_ascii_digit()) {
                return validate_cui(value.as_bytes());
            }
            if value.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
                validate_nit(value.as_bytes())
            } else {
                Err(ValidationError::InvalidFormat)
            }
        }
        "passport" => validate_passport(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}
