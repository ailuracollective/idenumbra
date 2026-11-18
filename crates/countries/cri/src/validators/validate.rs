/// DIMEX lengths in use: 11 (legacy) and 12 digits, both still valid per the
/// Ministerio de Hacienda e-invoice annexes.
const DIMEX_LENGTHS: [usize; 2] = [11, 12];

/// Validate a Costa Rican identification number (ID card/DIMEX/passport).
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.bytes().all(|b| b.is_ascii_digit()) {
        if value.bytes().all(|b| b == b'0') {
            return Err(ValidationError::InvalidFormat);
        }
        match value.len() {
            9 => Ok(()), // identity card
            len if DIMEX_LENGTHS.contains(&len) => Ok(()), // DIMEX
            _ => Err(ValidationError::InvalidLength {
                expected: 9,
                found: value.len(),
            }),
        }
    } else {
        // passport: alphanumeric 6..=12
        if value.len() < 6 || value.len() > 12 {
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
}
