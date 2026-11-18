/// Accepted numeric lengths. Edit when a document format is retired.
const ID_CARD_LEN: usize = 9;
/// Cédula jurídica and NITE: both are 10 characters, so the number alone does
/// not tell them apart.
const JURIDICAL_LEN: usize = 10;
/// DIMEX lengths in use: 11 (legacy) and 12 digits. The tax authority accepts
/// both and forbids a leading zero; the zero-padded 12-digit form exists only
/// inside the 50-character invoice key, not as a document number.
const DIMEX_LENGTHS: [usize; 2] = [11, 12];

/// Copy the characters of `value` into `buf`, dropping the separators people
/// type (`-`, `.`, ` `) and uppercasing letters. Reports the buffer length.
fn compact(value: &str, buf: &mut [u8]) -> Result<usize, ValidationError> {
    let mut n = 0usize;
    for (i, b) in value.bytes().enumerate() {
        if b.is_ascii_alphanumeric() {
            if n >= buf.len() {
                return Err(ValidationError::InvalidLength {
                    expected: buf.len(),
                    found: n + 1,
                });
            }
            buf[n] = b.to_ascii_uppercase();
            n += 1;
        } else if !matches!(b, b'-' | b'.' | b' ') {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(n)
}

/// Numeric documents: cédula física (9), cédula jurídica or NITE (10) and DIMEX
/// (11 or 12).
///
/// Hacienda's *Anexos y Estructuras* v4.4 forbids a leading zero for the cédula
/// física and the DIMEX only; it describes the NITE as "10 dígitos y sin
/// guiones" with no such rule. A 10-digit value may therefore start with `0`,
/// while the zero-padded 12-digit DIMEX used inside the invoice key stays
/// invalid as a document number.
fn validate_digits(digits: &[u8]) -> Result<(), ValidationError> {
    let len = digits.len();
    if len != ID_CARD_LEN && len != JURIDICAL_LEN && !DIMEX_LENGTHS.contains(&len) {
        return Err(ValidationError::InvalidLength {
            expected: ID_CARD_LEN,
            found: len,
        });
    }
    let all_zero = digits.iter().all(|d| *d == b'0');
    // Cédula física and DIMEX: no leading zero. NITE (10 digits): no such rule.
    if all_zero || (len != JURIDICAL_LEN && digits[0] == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Cédula jurídica: 1 digit for the type of legal person, 3 digits for its
/// class and 6 for the consecutive. The consecutive became alphanumeric in the
/// 2026 Registro Nacional change, so letters are accepted in the last six.
fn validate_juridical(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != JURIDICAL_LEN {
        return Err(ValidationError::InvalidLength {
            expected: JURIDICAL_LEN,
            found: chars.len(),
        });
    }
    if chars[0] == b'0'
        || !chars[..4].iter().all(u8::is_ascii_digit)
        || !chars[4..].iter().all(u8::is_ascii_alphanumeric)
    {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

fn validate_passport(chars: &[u8]) -> Result<(), ValidationError> {
    if !(6..=12).contains(&chars.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 6,
            found: chars.len(),
        });
    }
    Ok(())
}

/// Validate a Costa Rican identification number: cédula física, cédula
/// jurídica, NITE, DIMEX or passport.
///
/// No Costa Rican agency publishes a check digit for these documents, so only
/// the length, the alphabet and (for the cédula física and the DIMEX) the
/// no-leading-zero rule are checked.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    let mut buf = [0u8; 12];
    let n = compact(value, &mut buf)?;
    if n == 0 {
        return Err(ValidationError::InvalidLength {
            expected: ID_CARD_LEN,
            found: 0,
        });
    }
    let chars = &buf[..n];
    if chars.iter().all(u8::is_ascii_digit) {
        return validate_digits(chars);
    }
    if n == JURIDICAL_LEN {
        return validate_juridical(chars);
    }
    validate_passport(chars)
}
