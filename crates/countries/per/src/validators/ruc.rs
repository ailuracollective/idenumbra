/// SUNAT prefix pairs: 10/15/17 natural persons, 20 companies. Edit when a new
/// prefix is issued.
const RUC_PREFIXES: [&[u8]; 4] = [b"10", b"15", b"17", b"20"];

fn valid_ruc_prefix(prefix: &[u8]) -> bool {
    RUC_PREFIXES.contains(&prefix)
}

/// RUC check digit: modulo 11 with weights 5,4,3,2,7,6,5,4,3,2.
fn validate_ruc(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 11 {
        return Err(ValidationError::InvalidLength {
            expected: 11,
            found: chars.len(),
        });
    }
    if !chars.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if !valid_ruc_prefix(&chars[..2]) {
        return Err(ValidationError::InvalidFormat);
    }
    let weights = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = chars[..10]
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    // SUNAT: remainder 0 maps to 1, remainder 1 maps to 0.
    let computed = (11 - sum % 11) % 10;
    let received = (chars[10] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
