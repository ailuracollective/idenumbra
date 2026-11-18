fn validate_rut(chars: &[u8]) -> Result<(), ValidationError> {
    let (body, dv) = chars.split_at(chars.len() - 1);
    if body.is_empty() || body.len() > 8 {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: body.len(),
        });
    }
    if !body.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if body.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let mut sum = 0u32;
    let mut weight = 2u32;
    for d in body.iter().rev() {
        sum += (d - b'0') as u32 * weight;
        weight = if weight == 7 { 2 } else { weight + 1 };
    }
    let expected = match 11 - sum % 11 {
        11 => b'0',
        10 => b'K',
        digit => b'0' + digit as u8,
    };
    if dv[0] != expected {
        return Err(ValidationError::CheckDigitMismatch {
            expected: if expected == b'K' {
                10
            } else {
                (expected - b'0') as u32
            },
            received: if dv[0] == b'K' {
                10
            } else {
                (dv[0] - b'0') as u32
            },
        });
    }
    Ok(())
}
