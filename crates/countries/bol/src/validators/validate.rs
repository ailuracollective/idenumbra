/// Split the leading digit run from an optional CI complement suffix.
fn split_complement(value: &str) -> (usize, &str) {
    let end = value
        .bytes()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(value.len());
    (end, &value[end..])
}

/// A CI complement is one or two alphanumeric characters, optionally after a
/// separator.
fn valid_complement(rest: &str) -> bool {
    let rest = rest.trim_start_matches(['-', ' ']);
    (1..=2).contains(&rest.len()) && rest.bytes().all(|b| b.is_ascii_alphanumeric())
}

fn all_zero(digits: &[u8]) -> bool {
    !digits.is_empty() && digits.iter().all(|d| *d == b'0')
}

/// Validate a Bolivian identification number (CI or NIT).
///
/// Neither document has a published check digit: the CI is the registry number
/// and the NIT ends in a control code (`01`, `02`, `04`, plus `03` seen in the
/// SIN's supplier lists) plus one random digit, both assigned by the SIN.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let (digits_len, rest) = split_complement(value);
    if digits_len >= 9 && rest.is_empty() {
        return validate_nit(value.as_bytes());
    }
    if (5..=8).contains(&digits_len) && (rest.is_empty() || valid_complement(rest)) {
        if all_zero(&value.as_bytes()[..digits_len]) {
            return Err(ValidationError::InvalidFormat);
        }
        return Ok(()); // CI
    }
    if digits_len < 5 {
        return Err(ValidationError::InvalidLength {
            expected: 8,
            found: digits_len,
        });
    }
    Err(ValidationError::InvalidFormat)
}
