/// DIAN NIT check digit: modulo 11 with prime weights from the right.
fn validate_nit(digits: &[u8]) -> Result<(), ValidationError> {
    if !(2..=10).contains(&digits.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 10,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    let (body, dv) = digits.split_at(digits.len() - 1);
    if body.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [3u32, 7, 13, 17, 19, 23, 29, 37, 41];
    let mut sum = 0u32;
    for (i, d) in body.iter().rev().enumerate() {
        // body holds at most 9 digits, so every position has a weight.
        if let Some(w) = weights.get(i) {
            sum += (d - b'0') as u32 * w;
        }
    }
    let rest = sum % 11;
    let computed = if rest > 1 { 11 - rest } else { rest };
    let received = (dv[0] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
