/// Province codes run 1..=MAX_PROVINCE_CODE. Edit when provinces or comarcas
/// are created.
const MAX_PROVINCE_CODE: u32 = 13;

/// Markers written after the province code, or in its place: E extranjero,
/// N naturalizado, PE nacido en el exterior, AV antes de vigencia, PI población
/// indígena. Edit when the Tribunal Electoral issues a new one.
const ID_MARKERS: [&str; 5] = ["E", "N", "PE", "AV", "PI"];

/// Panamanian provinces are numbered 01-13.
fn valid_province(digits: &[u8]) -> bool {
    let province = digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32);
    (1..=MAX_PROVINCE_CODE).contains(&province)
}

/// First part of an ID: a rollo/tomo number, or a province code plus one of
/// `ID_MARKERS`.
fn valid_first_part(part: &str) -> bool {
    if part.is_empty() {
        return false;
    }
    let digits = part.bytes().take_while(u8::is_ascii_digit).count();
    if digits == part.len() {
        let value = part
            .bytes()
            .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32);
        return value != 0 && digits <= 9;
    }
    let province_ok = digits == 0
        || (digits <= 2 && valid_province(&part.as_bytes()[..digits]));
    province_ok && ID_MARKERS.contains(&&part[digits..])
}

fn all_digits(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit())
}

fn validate_hyphenated(value: &str) -> Result<(), ValidationError> {
    let mut parts = [""; 3];
    let mut count = 0usize;
    for part in value.split('-') {
        if count == parts.len() {
            return Err(ValidationError::InvalidFormat);
        }
        parts[count] = part;
        count += 1;
    }
    if count != 3 {
        return Err(ValidationError::InvalidFormat);
    }
    let (first, folio, asiento) = (parts[0], parts[1], parts[2]);
    if !valid_first_part(first)
        || !all_digits(folio)
        || folio.len() > 4
        || !all_digits(asiento)
        || asiento.len() > 9
    {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
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

/// Validate a Panamanian identification number (identity card or RUC).
///
/// The DGI RUC check digits depend on the taxpayer type, which cannot be
/// inferred from the number alone, so only the structure is validated.
pub fn validate(value: &str) -> Result<(), ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.contains('-') {
        return validate_hyphenated(value);
    }
    if all_digits(value) {
        if value.bytes().all(|b| b == b'0') {
            return Err(ValidationError::InvalidFormat);
        }
        return if (7..=13).contains(&value.len()) {
            Ok(())
        } else {
            Err(ValidationError::InvalidLength {
                expected: 10,
                found: value.len(),
            })
        };
    }
    validate_passport(value)
}
