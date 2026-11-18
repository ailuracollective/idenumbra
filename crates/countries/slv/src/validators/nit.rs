/// First digit of the NIT: 0-1 natural persons, 9 foreigners. Edit when DGII
/// adds a taxpayer type.
const TAXPAYER_TYPES: &[u8] = b"019";

/// Correlatives up to and including this value use the legacy weight table.
const LEGACY_CORRELATIVE_MAX: &[u8] = b"100";

/// NIT check digit: modulo 11, with a different weight set for old numbers.
fn validate_nit(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 14 {
        return Err(ValidationError::InvalidLength {
            expected: 14,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    // Nationals use 0/1; 9 is used for foreigners.
    if !TAXPAYER_TYPES.contains(&digits[0]) {
        return Err(ValidationError::InvalidFormat);
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    // The correlative decides which weight table applies (DGII rule).
    let old = &digits[10..13] <= LEGACY_CORRELATIVE_MAX;
    let old_weights = [14u32, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let new_weights = [2u32, 7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let weights: &[u32] = if old { &old_weights } else { &new_weights };
    let mut total = 0u32;
    for (d, w) in digits[..13].iter().zip(weights) {
        total += (d - b'0') as u32 * w;
    }
    let computed = if old {
        (total % 11) % 10
    } else {
        ((11 - total % 11) % 11) % 10
    };
    let received = (digits[13] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
