/// Validate a Guatemalan NIT.
///
/// SAT's FEL annex (`Reglas y validaciones` 2.1, §5.1.3): 2 to 12 characters,
/// the last one a check character from `0` to `9` or `K`, the rest digits.
/// Each body digit is multiplied by a factor that starts at `body length + 1`
/// and decreases; the check character is `(11 - sum mod 11) mod 11`, where `11`
/// is written as `0` and `10` as `K`.
///
/// Since 2023-10-04 a natural person's NIT is the first 9 digits of the CUI
/// (SAT FAQ), and since 2026-09-23 the full 13-digit CUI is also the NIT (CC
/// ruling on Resolución SAT-DSI-393-2025); `validate_with` routes those to
/// `validate_cui`. A 9-digit CUI prefix still satisfies this check, because the
/// CUI gives the same digit under mirrored weights.
fn validate_nit(input: &[u8]) -> Result<(), ValidationError> {
    // §5.1.1(d): the issuer should accept the hyphen people type right before
    // the check character ("1234567-9") and drop it before validating.
    let mut buffer = [0u8; 12];
    let len = input.len();
    let chars: &[u8] = if (3..=13).contains(&len) && input[len - 2] == b'-' {
        buffer[..len - 2].copy_from_slice(&input[..len - 2]);
        buffer[len - 2] = input[len - 1];
        &buffer[..len - 1]
    } else {
        input
    };
    if !(2..=12).contains(&chars.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 12,
            found: chars.len(),
        });
    }
    let (body, dv) = chars.split_at(chars.len() - 1);
    if let Some(index) = body.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    if body.iter().all(|d| *d == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    let dv = dv[0].to_ascii_uppercase();
    if !dv.is_ascii_digit() && dv != b'K' {
        return Err(ValidationError::InvalidFormat);
    }
    let mut factor = body.len() as u32 + 1;
    let mut total = 0u32;
    for d in body {
        total += (d - b'0') as u32 * factor;
        factor -= 1;
    }
    let modulus = (11 - total % 11) % 11;
    let computed = if modulus == 10 {
        b'K'
    } else {
        b'0' + modulus as u8
    };
    if dv != computed {
        return Err(ValidationError::CheckDigitMismatch {
            expected: if computed == b'K' {
                10
            } else {
                (computed - b'0') as u32
            },
            received: if dv == b'K' {
                10
            } else {
                (dv - b'0') as u32
            },
        });
    }
    Ok(())
}
