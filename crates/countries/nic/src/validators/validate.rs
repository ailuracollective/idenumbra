const LETTERS: &[u8; 23] = b"ABCDEFGHJKLMNPQRSTUVWXY";

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32)
}

/// Copy alphanumerics into `buf` uppercased, skipping hyphens and spaces.
fn compact(value: &str, buf: &mut [u8; 14]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: buf.len(),
                    found: n + 1,
                });
            }
            buf[n] = b.to_ascii_uppercase();
            n += 1;
        } else if !matches!(b, b'-' | b' ') {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(n)
}

/// Validate a Nicaraguan cédula.
///
/// Layout: municipio (3) + birth date DDMMYY (6) + serial (4) + letter. The
/// letter is the first 13 digits modulo 23, indexed in `LETTERS`. The CSE does
/// not publish the letter rule or the municipality codes, so only the letter
/// and the encoded birth date are checked here.
fn validate_id_card(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 14 {
        return Err(ValidationError::InvalidLength {
            expected: 14,
            found: chars.len(),
        });
    }
    if let Some(index) = chars[..13].iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    if !chars[13].is_ascii_alphabetic() {
        return Err(ValidationError::InvalidFormat);
    }
    let day = number(&chars[3..5]);
    let month = number(&chars[5..7]);
    // YY is read as 20YY, right for anyone cedulable today; it would only
    // misjudge 29 February 1900.
    let year = 2000 + number(&chars[7..9]);
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return Err(ValidationError::InvalidFormat);
    }
    let mut rest = 0u32;
    for d in &chars[..13] {
        rest = (rest * 10 + (d - b'0') as u32) % 23;
    }
    if chars[13] != LETTERS[rest as usize] {
        return Err(ValidationError::CheckDigitMismatch {
            expected: rest,
            received: LETTERS
                .iter()
                .position(|l| *l == chars[13])
                .unwrap_or(23) as u32,
        });
    }
    Ok(())
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

/// Validate a Nicaraguan identification number (identity card or passport).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 14];
    let n = compact(value, &mut buf)?;
    // A cédula is 13 digits plus a check letter, so 13 or 14 characters go to
    // the cédula path (and a shorter digit-only run is reported as a cédula
    // length error, not silently accepted as a passport).
    if n >= 13 || !value.bytes().any(|b| b.is_ascii_alphabetic()) {
        return validate_id_card(&buf[..n]);
    }
    validate_passport(value)
}
