const MAX_DIGITS: usize = 13;

/// Copy the digits of `value` into `buf`, dropping a hyphen written right before
/// the last digit (`171234567-5`). Any other hyphen is rejected.
fn digits(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let bytes = value.as_bytes();
    let mut n = 0;
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'-' && i + 2 == bytes.len() {
            continue;
        }
        if !b.is_ascii_digit() {
            return Err(ValidationError::NonNumeric { index: i });
        }
        if n == buf.len() {
            return Err(ValidationError::InvalidLength {
                expected: buf.len(),
                found: n + 1,
            });
        }
        buf[n] = *b;
        n += 1;
    }
    Ok(n)
}

/// Validate an Ecuadorian identification number.
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; MAX_DIGITS];
    match digits(value, &mut buf) {
        Ok(10) => {
            validate_id_card(&buf[..10])?;
            Ok(Validation {
                kind: "id_card",
                components: Some(Components::IdCard(id_card(&buf[..10]))),
            })
        }
        Ok(13) => {
            validate_ruc(&buf[..13])?;
            Ok(Validation {
                kind: "ruc",
                components: Some(Components::Ruc(ruc(&buf[..13]))),
            })
        }
        Ok(found) => Err(ValidationError::InvalidLength {
            expected: 10,
            found,
        }),
        Err(err) if value.bytes().all(|b| b.is_ascii_digit()) => Err(err),
        Err(_) => {
            validate_passport(value)?;
            Ok(Validation {
                kind: "passport",
                components: None,
            })
        }
    }
}