/// DIAN NIT check digit: modulo 11 with prime weights from the right.
///
/// Orden Administrativa 4/1989 (annex 3) publishes weights for a 9-digit body;
/// the 10th and 11th (43, 47) cover natural persons whose NIT is their cédula
/// or the 11-digit civil registry number, and are the continuation every
/// implementation uses.
fn validate_nit(digits: &[u8]) -> Result<(), ValidationError> {
    if !(2..=12).contains(&digits.len()) {
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
    let weights = [3u32, 7, 13, 17, 19, 23, 29, 37, 41, 43, 47];
    let mut sum = 0u32;
    for (i, d) in body.iter().rev().enumerate() {
        // `body` holds at most 11 digits, so every position has a weight.
        sum += (d - b'0') as u32 * weights[i];
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
