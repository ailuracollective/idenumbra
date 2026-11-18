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

/// Depurate an identification number the way SAT's FEL annex tells issuers to
/// (§5.1.1 for the NIT, §5.1.2 for the CUI): drop spaces and dots anywhere,
/// drop a hyphen or underscore only in the penultimate position, and uppercase
/// letters (so a trailing `k` becomes `K`).
fn depurate(value: &str, buf: &mut [u8; 14]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    let mut marks = [0usize; 14];
    for (i, b) in value.bytes().enumerate() {
        let b = match b {
            b' ' | b'.' => continue,
            b'a'..=b'z' => b - (b'a' - b'A'),
            b'0'..=b'9' | b'A'..=b'Z' => b,
            b'-' | b'_' => b,
            _ => return Err(ValidationError::NonAlphanumeric { index: i }),
        };
        if n >= buf.len() {
            return Err(ValidationError::InvalidLength {
                expected: buf.len(),
                found: n + 1,
            });
        }
        buf[n] = b;
        marks[n] = i;
        n += 1;
    }
    // §5.1.1(d): the only hyphen/underscore that may appear is the one people
    // type right before the check character.
    if n >= 2 && matches!(buf[n - 2], b'-' | b'_') {
        buf[n - 2] = buf[n - 1];
        marks[n - 2] = marks[n - 1];
        n -= 1;
    }
    for j in 0..n {
        if matches!(buf[j], b'-' | b'_') {
            return Err(ValidationError::NonAlphanumeric { index: marks[j] });
        }
    }
    Ok(n)
}

/// Validate a Guatemalan identification number (CUI/DPI or NIT).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 14];
    let n = depurate(value, &mut buf)?;
    let bytes = &buf[..n];
    if bytes.iter().all(u8::is_ascii_digit) {
        // SAT §5.1.4.1: a CUI has 12 or 13 digits. A 12-digit CUI (a
        // correlativo that lost its leading zero) shares the 2-12 digit NIT
        // shape, so try the more specific CUI layout first and fall back to
        // the NIT rule.
        return match n {
            13 => validate_cui(bytes),
            12 => validate_cui(bytes).or_else(|_| validate_nit(bytes)),
            _ => validate_nit(bytes),
        };
    }
    if n >= 2 && bytes[n - 1] == b'K' && bytes[..n - 1].iter().all(u8::is_ascii_digit) {
        return validate_nit(bytes);
    }
    validate_passport(value)
}
