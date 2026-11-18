/// Paraguay RUC check digit: modulo 11 with weights 2..11 from the right.
fn validate_ruc(digits: &[u8]) -> Result<(), ValidationError> {
    // SIFEN: the RUC without the check digit is 3-8 characters.
    if !(4..=9).contains(&digits.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 9,
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
    let mut sum = 0u32;
    let mut weight = 2u32;
    for d in body.iter().rev() {
        sum += (d - b'0') as u32 * weight;
        weight = if weight == 11 { 2 } else { weight + 1 };
    }
    let rest = sum % 11;
    // SET: 11 - remainder, with 0 and 1 mapping to 0.
    let computed = if rest > 1 { 11 - rest } else { 0 };
    let received = (dv[0] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
