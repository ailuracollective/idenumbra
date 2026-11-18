/// Validate a Guatemalan NIT (2-12 digits, check character 0-9 or K).
fn validate_nit(chars: &[u8]) -> Result<(), ValidationError> {
    if !(2..=12).contains(&chars.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 12,
            found: chars.len(),
        });
    }
    let (body, dv) = chars.split_at(chars.len() - 1);
    if !body.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if body.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let dv = dv[0].to_ascii_uppercase();
    if !dv.is_ascii_digit() && dv != b'K' {
        return Err(ValidationError::InvalidFormat);
    }
    let mut factor = body.len() as u32 + 1;
    let mut total = 0u32;
    for d in body {
        total += (d - b'0') as u32 * factor;
        factor -= 1;
    }
    let modulus = (11 - total % 11) % 11;
    let computed = if modulus == 10 {
        b'K'
    } else {
        b'0' + modulus as u8
    };
    if dv != computed {
        return Err(ValidationError::CheckDigitMismatch {
            expected: if computed == b'K' {
                10
            } else {
                (computed - b'0') as u32
            },
            received: if dv == b'K' {
                10
            } else {
                (dv - b'0') as u32
            },
        });
    }
    Ok(())
}
