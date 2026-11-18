const MAX_DIGITS: usize = 13;

/// Copy the digits of `value` into `buf`, dropping a hyphen written right before
/// the last digit (`171234567-5`). Any other hyphen is rejected.
fn digits(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let bytes = value.as_bytes();
    let mut n = 0;
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'-' && i + 2 == bytes.len() {
            continue;
        }
        if !b.is_ascii_digit() {
            return Err(ValidationError::NonNumeric { index: i });
        }
        if n == buf.len() {
            return Err(ValidationError::InvalidLength {
                expected: buf.len(),
                found: n + 1,
            });
        }
        buf[n] = *b;
        n += 1;
    }
    Ok(n)
}

/// Validate by digit count: cédula (10) or RUC (13).
fn validate_by_length(digits: &[u8]) -> Result<(), ValidationError> {
    match digits.len() {
        10 => validate_id_card(digits),
        13 => validate_ruc(digits),
        found => Err(ValidationError::InvalidLength {
            expected: 10,
            found,
        }),
    }
}

/// Validate an Ecuadorian identification number.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; MAX_DIGITS];
    match digits(value, &mut buf) {
        Ok(len) => validate_by_length(&buf[..len]),
        // A digit-only run that is too long is a cédula/RUC, not a passport.
        Err(err) if value.bytes().all(|b| b.is_ascii_digit()) => Err(err),
        Err(_) => validate_passport(value),
    }
}
