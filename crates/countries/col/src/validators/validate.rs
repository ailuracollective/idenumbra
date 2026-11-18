/// Copy ASCII digits into `buf`, skipping dots, hyphens and spaces.
fn compact_digits(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_digit() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: buf.len(),
                    found: n + 1,
                });
            }
            buf[n] = b;
            n += 1;
        } else if !matches!(b, b'.' | b'-' | b' ') {
            return Err(ValidationError::NonNumeric { index: i });
        }
    }
    Ok(n)
}

/// Validate a Colombian cédula de ciudadanía or passport.
///
/// Cédulas carry no check digit, so a 10-digit NIT cannot be told apart from
/// a 10-digit cédula by shape alone; use `validate_with(.., Some("nit"))` to
/// check the NIT check digit.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().all(|b| b.is_ascii_digit()) {
        if value.bytes().all(|b| b == b'0') {
            return Err(ValidationError::InvalidFormat);
        }
        return if (6..=10).contains(&value.len()) {
            Ok(())
        } else {
            Err(ValidationError::InvalidLength {
                expected: 10,
                found: value.len(),
            })
        };
    }
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 12,
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
