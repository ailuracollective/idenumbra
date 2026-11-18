/// First digit of the NIT: 0-1 natural persons, 9 foreigners. Edit when DGII
/// adds a taxpayer type.
const TAXPAYER_TYPES: &[u8] = b"019";

/// Correlatives up to and including this value use the legacy weight table.
const LEGACY_CORRELATIVE_MAX: u32 = 100;

/// Legacy table: weights 14..2, digit `(sum mod 11) mod 10`.
fn legacy_check_digit(digits: &[u8]) -> u32 {
    const WEIGHTS: [u32; 13] = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut total = 0u32;
    for (d, w) in digits[..13].iter().zip(WEIGHTS) {
        total += (d - b'0') as u32 * w;
    }
    (total % 11) % 10
}

/// Current table: weights `2,7,6,5,4,3,2` repeated, digit `(11 - sum mod 11)`
/// with 10 and 11 collapsing to 0.
fn current_check_digit(digits: &[u8]) -> u32 {
    const WEIGHTS: [u32; 13] = [2, 7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let mut total = 0u32;
    for (d, w) in digits[..13].iter().zip(WEIGHTS) {
        total += (d - b'0') as u32 * w;
    }
    ((11 - total % 11) % 11) % 10
}

/// NIT check digit: modulo 11, with a different weight set for old numbers.
fn validate_nit(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 14 {
        return Err(ValidationError::InvalidLength {
            expected: 14,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    // Nationals use 0/1; 9 is used for foreigners.
    if !TAXPAYER_TYPES.contains(&digits[0]) {
        return Err(ValidationError::InvalidFormat);
    }
    if digits.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    // The correlative (positions 11-13) selects the weight table. The cut-off
    // is reverse engineered: see the README for the public samples behind it.
    let correlative = (digits[10] - b'0') as u32 * 100
        + (digits[11] - b'0') as u32 * 10
        + (digits[12] - b'0') as u32;
    let legacy_correlative = correlative <= LEGACY_CORRELATIVE_MAX;
    let legacy = legacy_check_digit(digits);
    let current = current_check_digit(digits);
    let expected = if legacy_correlative { legacy } else { current };
    let received = (digits[13] - b'0') as u32;
    // Every public sample with a correlative of 101 or more uses the current
    // table, but no sample in the 014-100 band discriminates between the two
    // (the two known ones at 014 match both tables or neither), so either
    // table is accepted there instead of rejecting numbers that may be
    // legitimately issued.
    if received == expected || (legacy_correlative && received == current) {
        return Ok(());
    }
    Err(ValidationError::CheckDigitMismatch { expected, received })
}
