// AFIP/ARCA prefixes (20/23/24/25/26/27 for people, 30/33/34 companies,
// 50/51/55 foreign) are not enforced: since 2026 ARCA assigns the prefix
// randomly to new CUIT/CUIL numbers, so only the check digit is reliable.
fn validate_cuit(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 11 {
        return Err(ValidationError::InvalidLength {
            expected: 11,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..10]
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    // AFIP maps remainder 1 to check digit 9 and remainder 0 to 0.
    let computed = match 11 - sum % 11 {
        11 => 0,
        10 => 9,
        digit => digit,
    };
    let received = (digits[10] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
