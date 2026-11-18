/// NIT lengths assigned by the SIN (examples range from 7 to 12 digits). Edit
/// when the SIN changes the numbering.
const NIT_MIN_LEN: usize = 7;
const NIT_MAX_LEN: usize = 12;

/// Validate the structure of a Bolivian NIT.
///
/// The SIN assigns three control digits to every NIT and does not publish the
/// algorithm: the SIAT "Módulo 11" page documents the CUF (invoice code), not
/// the NIT. Only the shape is checked, never a check digit.
fn validate_nit(digits: &[u8]) -> Result<(), ValidationError> {
    if !(NIT_MIN_LEN..=NIT_MAX_LEN).contains(&digits.len()) {
        return Err(ValidationError::InvalidLength {
            expected: NIT_MAX_LEN,
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
