/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    let mut buf = [0u8; 12];
    let n = compact(value, &mut buf)?;
    let chars = &buf[..n];
    match kind {
        "id_card" => {
            if n != ID_CARD_LEN {
                return Err(ValidationError::InvalidLength {
                    expected: ID_CARD_LEN,
                    found: n,
                });
            }
            if !chars.iter().all(u8::is_ascii_digit) {
                return Err(ValidationError::InvalidFormat);
            }
            validate_digits(chars)
        }
        "cedula_juridica" => validate_juridical(chars),
        "dimex" => {
            if !DIMEX_LENGTHS.contains(&n) {
                return Err(ValidationError::InvalidLength {
                    expected: 12,
                    found: n,
                });
            }
            if !chars.iter().all(u8::is_ascii_digit) {
                return Err(ValidationError::InvalidFormat);
            }
            validate_digits(chars)
        }
        "passport" => validate_passport(chars),
        _ => Err(ValidationError::InvalidFormat),
    }
}
