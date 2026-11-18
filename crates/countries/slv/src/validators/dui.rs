/// DUI check digit: modulo 10 with weights 9..2.
fn validate_dui(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 9 {
        return Err(ValidationError::InvalidLength {
            expected: 9,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    // A DUI made only of zeros does not exist.
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let mut sum = 0u32;
    for (i, d) in digits[..8].iter().enumerate() {
        sum += (d - b'0') as u32 * (9 - i as u32);
    }
    let computed = (10 - sum % 10) % 10;
    let received = (digits[8] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
