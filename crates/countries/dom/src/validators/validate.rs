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

/// Validate a Dominican cédula: Luhn (mod 10) over 11 digits.
fn validate_id_card(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 11 || !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidLength {
            expected: 11,
            found: digits.len(),
        });
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let mut sum = 0u32;
    for (i, &b) in digits[..10].iter().rev().enumerate() {
        let mut d = (b - b'0') as u32;
        if i % 2 == 0 {
            d *= 2;
            if d > 9 {
                d -= 9;
            }
        }
        sum += d;
    }
    let computed = (10 - sum % 10) % 10;
    let received = (digits[10] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}

/// Validate a Dominican identification number (identity card or passport).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 11];
    let mut n = 0usize;
    for b in value.bytes() {
        if b.is_ascii_digit() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: 11,
                    found: n + 1,
                });
            }
            buf[n] = b;
            n += 1;
        } else if !matches!(b, b'-' | b' ') {
            return validate_passport(value);
        }
    }
    validate_id_card(&buf[..n])
}
