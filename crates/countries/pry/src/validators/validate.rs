/// Copy digits into `buf`, skipping dots, hyphens and spaces.
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

/// Validate a Paraguayan identification number (CI or RUC).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 9];
    let n = match compact_digits(value, &mut buf) {
        Ok(n) => n,
        // Anything that is not a digit or a separator may be a passport.
        Err(ValidationError::NonNumeric { .. }) => return validate_passport(value),
        Err(err) => return Err(err),
    };
    match n {
        6 | 7 => {
            if buf[..n].iter().all(|d| *d == b'0') {
                return Err(ValidationError::InvalidFormat);
            }
            Ok(()) // CI
        }
        8 | 9 => validate_ruc(&buf[..n]),
        _ => Err(ValidationError::InvalidLength {
            expected: 7,
            found: n,
        }),
    }
}
