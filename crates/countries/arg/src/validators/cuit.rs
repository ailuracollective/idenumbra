// The type prefix is not enforced: RG Conjunta AFIP-ANSES 5007/2021 (BO
// 10/06/2021) assigns 20/23/24/27, "or those determined in the future", at
// random to new CUIT/CUIL numbers, and 25/26 are contingency prefixes. Only the
// check digit is reliable.
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
    // Public implementations map remainder 1 (11 - 1 = 10) to check digit 9 and
    // remainder 0 to 0; ARCA publishes no algorithm, so this is the convention.
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
