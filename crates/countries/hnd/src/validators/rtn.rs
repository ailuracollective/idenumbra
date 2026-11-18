/// DNI and RTN lengths. Edit when the RNP changes them.
const DNI_LEN: usize = 13;
const RTN_LEN: usize = 14;

/// Validate the structure of a Honduran RTN.
///
/// For natural persons the RTN is the 13-digit DNI plus one digit, but the SAR
/// does not publish the verifier algorithm, so only the shape is checked.
fn validate_rtn(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != RTN_LEN {
        return Err(ValidationError::InvalidLength {
            expected: RTN_LEN,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if all_zero(digits) {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

fn validate_dni(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != DNI_LEN {
        return Err(ValidationError::InvalidLength {
            expected: DNI_LEN,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    if all_zero(digits) {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}
