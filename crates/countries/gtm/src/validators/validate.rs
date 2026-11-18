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

/// Whether `bytes` has the shape of a NIT as people type it: digits, an
/// optional hyphen right before the check character, and a final digit or `K`.
fn nit_shaped(bytes: &[u8]) -> bool {
    let Some(&dv) = bytes.last() else {
        return false;
    };
    let body = &bytes[..bytes.len() - 1];
    let body = if body.last() == Some(&b'-') {
        &body[..body.len() - 1]
    } else {
        body
    };
    !body.is_empty()
        && body.iter().all(u8::is_ascii_digit)
        && (dv.is_ascii_digit() || matches!(dv, b'K' | b'k'))
}

/// Validate a Guatemalan identification number (CUI/DPI or NIT).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let bytes = value.as_bytes();
    if bytes.iter().all(u8::is_ascii_digit) {
        // A 12-digit CUI (a correlativo that lost its leading zero) shares the
        // 2-12 digit NIT shape; only `validate_with(.., "cui")` tells them apart.
        return if bytes.len() == 13 {
            validate_cui(bytes)
        } else {
            validate_nit(bytes)
        };
    }
    // SAT §5.1.1(d): the printed NIT keeps a hyphen before the check character.
    if nit_shaped(bytes) {
        return validate_nit(bytes);
    }
    validate_passport(value)
}
