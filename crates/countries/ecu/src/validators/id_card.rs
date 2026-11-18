/// Highest province code issued (01-24). Edit when provinces split or merge.
const MAX_PROVINCE_CODE: u32 = 24;

/// Code for people whose birth was registered abroad (Registro Civil,
/// Resolución 140, Registro Oficial 681 of 2012).
const ABROAD_PROVINCE_CODE: u32 = 30;

fn number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32)
}

fn all_zero(digits: &[u8]) -> bool {
    digits.iter().all(|d| *d == b'0')
}

/// Validate an Ecuadorian cédula (10 digits, modulo 10).
fn validate_id_card(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 10 {
        return Err(ValidationError::InvalidLength {
            expected: 10,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    let province = number(&digits[0..2]);
    if !(1..=MAX_PROVINCE_CODE).contains(&province) && province != ABROAD_PROVINCE_CODE {
        return Err(ValidationError::InvalidRegion { region: province });
    }
    // Natural persons use 0-5 as the third digit.
    if digits[2] > b'5' {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [2u32, 1, 2, 1, 2, 1, 2, 1, 2];
    let mut sum = 0u32;
    for (d, w) in digits[..9].iter().zip(weights) {
        let mut p = (d - b'0') as u32 * w;
        if p >= 10 {
            p -= 9;
        }
        sum += p;
    }
    let computed = (10 - sum % 10) % 10;
    let received = (digits[9] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
