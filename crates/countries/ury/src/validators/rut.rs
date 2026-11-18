/// First two digits: the DGI operating-unit code, 1..=22. 01-21 are historical
/// and 22 is used for every new RUT since October 2024. Edit when the DGI
/// changes the prefix.
const MAX_OFFICE_CODE: u32 = 22;

/// Middle group, a constant per the DGI; `001` in every public sample.
const RUT_OFFICE_GROUP: &[u8] = b"001";

/// Validate a Uruguayan RUT (12 digits).
fn validate_rut(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 12 {
        return Err(ValidationError::InvalidLength {
            expected: 12,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    let first = (digits[0] - b'0') as u32 * 10 + (digits[1] - b'0') as u32;
    if !(1..=MAX_OFFICE_CODE).contains(&first) {
        return Err(ValidationError::InvalidRegion { region: first });
    }
    if digits[2..8].iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    if &digits[8..11] != RUT_OFFICE_GROUP {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [4u32, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..11]
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    let rest = sum % 11;
    let computed = if rest == 0 { 0 } else { 11 - rest };
    if computed >= 10 {
        return Err(ValidationError::UnrepresentableCheckDigit);
    }
    let received = (digits[11] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
