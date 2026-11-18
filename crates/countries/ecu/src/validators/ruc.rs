/// Provisional series issued before 2010, per the Registro Civil; still seen
/// on legacy RUCs.
const PROVISIONAL_CODE: u32 = 50;

/// True when the weighted sum of body + check digit is a multiple of 11.
fn mod11(digits_with_dv: &[u8], weights: &[u32]) -> Result<(), ValidationError> {
    let mut sum = 0u32;
    for (d, w) in digits_with_dv.iter().zip(weights) {
        sum += (d - b'0') as u32 * w;
    }
    if sum % 11 == 0 {
        return Ok(());
    }
    let body = &digits_with_dv[..digits_with_dv.len() - 1];
    let body_sum: u32 = body
        .iter()
        .zip(weights)
        .map(|(d, w)| (d - b'0') as u32 * w)
        .sum();
    let expected = (11 - body_sum % 11) % 11;
    if expected >= 10 {
        return Err(ValidationError::UnrepresentableCheckDigit);
    }
    Err(ValidationError::CheckDigitMismatch {
        expected,
        received: (digits_with_dv[digits_with_dv.len() - 1] - b'0') as u32,
    })
}

/// Natural person RUC: valid cédula plus a non-zero establishment number.
fn validate_natural(digits: &[u8]) -> Result<(), ValidationError> {
    if all_zero(&digits[10..13]) {
        return Err(ValidationError::InvalidFormat);
    }
    validate_id_card(&digits[..10])
}

/// Private company RUC: modulo 11 over the first 10 odd-position digits.
fn validate_juridical(digits: &[u8]) -> Result<(), ValidationError> {
    if all_zero(&digits[10..13]) {
        return Err(ValidationError::InvalidFormat);
    }
    mod11(&digits[..10], &[4, 3, 2, 7, 6, 5, 4, 3, 2, 1])
}

/// Public company RUC: modulo 11 over the first 9 even-position digits.
fn validate_public(digits: &[u8]) -> Result<(), ValidationError> {
    if all_zero(&digits[9..13]) {
        return Err(ValidationError::InvalidFormat);
    }
    mod11(&digits[..9], &[3, 2, 7, 6, 5, 4, 3, 2, 1])
}

/// Validate an Ecuadorian RUC (13 digits).
fn validate_ruc(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 13 {
        return Err(ValidationError::InvalidLength {
            expected: 13,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    let province = number(&digits[0..2]);
    if !(1..=MAX_PROVINCE_CODE).contains(&province)
        && !matches!(province, ABROAD_PROVINCE_CODE | PROVISIONAL_CODE)
    {
        return Err(ValidationError::InvalidRegion { region: province });
    }
    // SRI allows the 6 and 9 prefixes for either company or natural persons.
    match digits[2] {
        b'0'..=b'5' => validate_natural(digits),
        b'6' => validate_public(digits).or_else(|_| validate_natural(digits)),
        b'9' => validate_public(digits).or_else(|_| validate_juridical(digits)),
        _ => Err(ValidationError::InvalidFormat),
    }
}
