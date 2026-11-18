fn validate_cnpj(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 14 {
        return Err(ValidationError::InvalidLength {
            expected: 14,
            found: chars.len(),
        });
    }
    // Since July 2026 the root and branch may contain letters; the check
    // digits are always numeric.
    if !chars[..12].iter().all(u8::is_ascii_alphanumeric)
        || !chars[12..].iter().all(u8::is_ascii_digit)
    {
        return Err(ValidationError::InvalidFormat);
    }
    if chars.iter().all(|b| *b == chars[0]) {
        return Err(ValidationError::InvalidFormat);
    }
    let w1 = [5u32, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;
    for (i, &b) in chars[..12].iter().enumerate() {
        sum += char_val(b) * w1[i];
    }
    let rest = sum % 11;
    let dv1 = if rest < 2 { 0 } else { 11 - rest };
    let received1 = (chars[12] - b'0') as u32;
    if dv1 != received1 {
        return Err(ValidationError::CheckDigitMismatch {
            expected: dv1,
            received: received1,
        });
    }
    let w2 = [6u32, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut sum = 0u32;
    for (i, &b) in chars[..13].iter().enumerate() {
        sum += char_val(b) * w2[i];
    }
    let rest = sum % 11;
    let dv2 = if rest < 2 { 0 } else { 11 - rest };
    let received2 = (chars[13] - b'0') as u32;
    if dv2 != received2 {
        return Err(ValidationError::CheckDigitMismatch {
            expected: dv2,
            received: received2,
        });
    }
    Ok(())
}
