/// Validate a Peruvian identification number (DNI, RUC or foreign ID).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().all(|b| b.is_ascii_digit()) {
        return match value.len() {
            8 => {
                if value.bytes().all(|b| b == b'0') {
                    return Err(ValidationError::InvalidFormat);
                }
                Ok(()) // DNI
            }
            11 => validate_ruc(value.as_bytes()),
            found => Err(ValidationError::InvalidLength {
                expected: 8,
                found,
            }),
        };
    }
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: value.len(),
        });
    }
    for (i, b) in value.bytes().enumerate() {
        if !b.is_ascii_alphanumeric() {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(())
}
