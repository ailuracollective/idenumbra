/// Code `50` is not a province: it appears in the python-stdnum prefix list
/// (legacy RUCs) but not in the SRI Reglamento or the Registro Civil table.
/// Kept so those RUCs are not rejected; remove when the SRI confirms it.
const LEGACY_CODE: u32 = 50;

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

/// Natural person RUC: a valid cédula plus the fixed suffix `001` (Reglamento
/// to the RUC law, art. 3).
fn validate_natural(digits: &[u8]) -> Result<(), ValidationError> {
    if digits[10..13] != *b"001" {
        return Err(ValidationError::InvalidFormat);
    }
    validate_id_card(&digits[..10])
}

/// Public entity RUC: modulo 11 over the 8-digit body, tenth digit always `0`,
/// last four `0001` (SRI structure published through the OECD).
fn validate_public(digits: &[u8]) -> Result<(), ValidationError> {
    if digits[9..13] != *b"0001" {
        return Err(ValidationError::InvalidFormat);
    }
    mod11(&digits[..9], &[3, 2, 7, 6, 5, 4, 3, 2, 1])
}

/// Private company or foreign person RUC. The SRI stopped guaranteeing the
/// modulo 11 digit once the sequential passes six digits and recommends
/// querying the catastro instead, so only the structure is checked here.
fn validate_private(digits: &[u8]) -> Result<(), ValidationError> {
    if digits[10..13] != *b"001" {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Validate an Ecuadorian RUC (13 digits).
fn validate_ruc(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 13 {
        return Err(ValidationError::InvalidLength {
            expected: 13,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    let province = number(&digits[0..2]);
    if !(1..=MAX_PROVINCE_CODE).contains(&province)
        && !matches!(province, ABROAD_PROVINCE_CODE | LEGACY_CODE)
    {
        return Err(ValidationError::InvalidRegion { region: province });
    }
    match digits[2] {
        b'0'..=b'5' => validate_natural(digits),
        b'6' => validate_public(digits),
        b'9' => validate_private(digits),
        _ => Err(ValidationError::InvalidFormat),
    }
}
