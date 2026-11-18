fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 6,
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

/// Validate a Guatemalan identification number (CUI/DPI or NIT).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().all(|b| b.is_ascii_digit()) {
        return if value.len() == 13 {
            validate_cui(value.as_bytes())
        } else {
            validate_nit(value.as_bytes())
        };
    }
    // NIT check characters may be K, as in 1234567K.
    if value
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_digit())
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'K' | b'k'))
    {
        return validate_nit(value.as_bytes());
    }
    validate_passport(value)
}
