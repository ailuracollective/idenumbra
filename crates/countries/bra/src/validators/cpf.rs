fn validate_cpf(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 11 {
        return Err(ValidationError::InvalidLength {
            expected: 11,
            found: chars.len(),
        });
    }
    if !chars.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    // Repeated digits pass the arithmetic but are never issued.
    if chars.iter().all(|b| *b == chars[0]) {
        return Err(ValidationError::InvalidFormat);
    }
    let mut sum = 0u32;
    for (i, &b) in chars[..9].iter().enumerate() {
        sum += (b - b'0') as u32 * (10 - i as u32);
    }
    let rest = sum % 11;
    let dv1 = if rest < 2 { 0 } else { 11 - rest };
    let received1 = (chars[9] - b'0') as u32;
    if dv1 != received1 {
        return Err(ValidationError::CheckDigitMismatch {
            expected: dv1,
            received: received1,
        });
    }
    let mut sum = 0u32;
    for (i, &b) in chars[..10].iter().enumerate() {
        sum += (b - b'0') as u32 * (11 - i as u32);
    }
    let rest = sum % 11;
    let dv2 = if rest < 2 { 0 } else { 11 - rest };
    let received2 = (chars[10] - b'0') as u32;
    if dv2 != received2 {
        return Err(ValidationError::CheckDigitMismatch {
            expected: dv2,
            received: received2,
        });
    }
    Ok(())
}
