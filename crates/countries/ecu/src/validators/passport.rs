fn validate_passport(s: &str) -> Result<(), ValidationError> {
    if s.len() < 6 || s.len() > 9 {
        return Err(ValidationError::InvalidLength {
            expected: 6,
            found: s.len(),
        });
    }
    for (i, b) in s.bytes().enumerate() {
        if !b.is_ascii_alphanumeric() {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(())
}
