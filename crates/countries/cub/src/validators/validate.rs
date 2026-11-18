/// Century code (7th digit): 9 = 18xx, 0-5 = 19xx, 6-8 = 20xx. Edit when a new
/// century is added.
const CENTURY_19TH: u8 = 9;
const CENTURY_20TH_MAX: u8 = 5;

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

/// Validate a Cuban identity card. The layout is YYMMDD + century code +
/// serial (2) + sex (1) + control digit. The control digit algorithm is not
/// public, so only the encoded birth date is checked here.
///
/// The 7th digit marks the century, it is not a range check on the two year
/// digits: 19xx numbers use 0-5 for any year (1972 -> 0, 1991 -> 2, 1999 -> 5).
/// The sex digit (10th) is even for men and odd for women. Every digit is even
/// or odd, so it constrains nothing: without the person's sex there is no way
/// to reject a number on that position, and the MININT reassigns the number
/// when the registered sex changes.
fn validate_id_card(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 11 || !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidLength {
            expected: 11,
            found: digits.len(),
        });
    }
    let yy = number(&digits[0..2]);
    let month = number(&digits[2..4]);
    let day = number(&digits[4..6]);
    // 7th digit: 9 = 18xx, 0-5 = 19xx, 6-8 = 20xx.
    let century = digits[6] - b'0';
    let year = (if century == CENTURY_19TH {
        1800
    } else if century <= CENTURY_20TH_MAX {
        1900
    } else {
        2000
    }) + yy;
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Validate a Cuban identification number (identity card or passport).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().all(|b| b.is_ascii_digit()) {
        if value.len() != 11 {
            return Err(ValidationError::InvalidLength {
                expected: 11,
                found: value.len(),
            });
        }
        return validate_id_card(value.as_bytes());
    }
    validate_passport(value)
}
