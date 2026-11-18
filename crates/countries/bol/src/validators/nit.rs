/// NIT lengths seen in public supplier lists (9-11 digits). The SIN publishes
/// no length bound; the range stays permissive. Edit when the SIN documents one.
const NIT_MIN_LEN: usize = 7;
const NIT_MAX_LEN: usize = 12;

/// Control codes the SIN appends after the CI: `01` natural person or
/// unipersonal company, `02` legal person, `04` foreign person. Edit when the
/// SIN publishes a new code on its "Generación del NIT" page.
const NIT_CONTROL_CODES: [u8; 3] = [1, 2, 4];

/// Validate the structure of a Bolivian NIT.
///
/// Per the SIN/SIAT, a natural person's NIT is the CI followed by `01` and one
/// random digit; legal persons get `02` and foreigners `04` in the same place.
/// Those last three digits are control codes, not a check digit: the SIN
/// publishes no verifier algorithm, and the SIAT "Módulo 11" page computes the
/// CUF (invoice code), not the NIT. The random digit itself is not restricted
/// (public lists contain 0).
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
    let code = digits.len() - 3;
    if digits[code] != b'0' || !NIT_CONTROL_CODES.contains(&(digits[code + 1] - b'0')) {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}
