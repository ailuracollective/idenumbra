/// DNI and RTN lengths. Edit when the RNP changes the DNI format or the SAR
/// changes the RTN format.
const DNI_LEN: usize = 13;
const RTN_LEN: usize = 14;

/// Validate the structure of a Honduran RTN.
///
/// The SAR states that a natural person's RTN is the DNI plus one digit, and
/// that the RTN has been a 14-digit number since 2003. Neither the SAR nor the
/// SEFIN/SDE publishes the verifier algorithm, so only the shape is checked
/// (tested against 124 supplier RTNs from ONCAE open data: no check digit).
fn validate_rtn(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != RTN_LEN {
        return Err(ValidationError::InvalidLength {
            expected: RTN_LEN,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
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
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    if all_zero(digits) {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}
