/// Copy RFC/CURP characters into `buf`, dropping separators and folding case.
fn compact(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, c) in value.char_indices() {
        let b = match c {
            '0'..='9' => c as u8,
            'a'..='z' => c.to_ascii_uppercase() as u8,
            'A'..='Z' => c as u8,
            '&' => b'&',
            'ñ' | 'Ñ' => ENYE,
            '-' | ' ' | '_' => continue,
            _ => return Err(ValidationError::NonAlphanumeric { index: i }),
        };
        if n >= buf.len() {
            return Err(ValidationError::InvalidLength {
                expected: buf.len(),
                found: n + 1,
            });
        }
        buf[n] = b;
        n += 1;
    }
    Ok(n)
}

/// Validate a Mexican identification number (CURP or RFC).
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 18];
    let n = compact(value, &mut buf)?;
    match n {
        18 => {
            validate_curp(&buf[..18])?;
            Ok(Validation {
                kind: "curp",
                components: Some(Components::Curp(curp(&buf[..18]))),
            })
        }
        12 | 13 => {
            validate_rfc(&buf[..n])?;
            Ok(Validation {
                kind: "rfc",
                components: Some(Components::Rfc(rfc(&buf[..n]))),
            })
        }
        _ => Err(ValidationError::InvalidLength {
            expected: 18,
            found: n,
        }),
    }
}
