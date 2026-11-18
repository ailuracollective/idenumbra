/// Sentinel used to keep Ñ as a single byte while parsing UTF-8 input.
const ENYE: u8 = 0x80;

/// Character values for the RFC checksum: the public table
/// `0123456789ABCDEFGHIJKLMN&OPQRSTUVWXYZ Ñ` (space = 37, Ñ = 38). The SAT
/// documents the RFC shape and the check digit's existence but not the formula.
fn rfc_value(b: u8) -> u32 {
    match b {
        b'0'..=b'9' => (b - b'0') as u32,
        b'A'..=b'N' => 10 + (b - b'A') as u32,
        b'&' => 24,
        b'O'..=b'Z' => 25 + (b - b'O') as u32,
        b' ' => 37,
        _ => 38,
    }
}

/// RFCs the SAT uses as generic values: no RFC at all (public at large) and a
/// resident abroad not registered in the RFC. RMF 2026, rule 2.7.1.23.
const GENERIC_RFCS: [&[u8]; 2] = [b"XAXX010101000", b"XEXX010101000"];

fn is_generic_rfc(chars: &[u8]) -> bool {
    GENERIC_RFCS.contains(&chars)
}

/// Validate a Mexican RFC (12 chars for companies, 13 for persons). The
/// 10-character "RFC sin homoclave" of some legacy systems is rejected: the SAT
/// only accepts 12/13 positions in CFDI and nómina.
fn validate_rfc(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 12 && chars.len() != 13 {
        return Err(ValidationError::InvalidLength {
            expected: 13,
            found: chars.len(),
        });
    }
    if is_generic_rfc(chars) {
        return Ok(());
    }
    let letters = chars.len() - 9;
    let date_at = letters;
    for c in &chars[..letters] {
        if !(c.is_ascii_uppercase() || *c == b'&' || *c == ENYE) {
            return Err(ValidationError::InvalidFormat);
        }
    }
    if !chars[date_at..date_at + 6]
        .iter()
        .all(u8::is_ascii_digit)
    {
        return Err(ValidationError::InvalidFormat);
    }
    if !chars[date_at + 6..]
        .iter()
        .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase())
    {
        return Err(ValidationError::InvalidFormat);
    }
    // The SAT does not encode the century, so dates are checked as 20xx. This
    // only matters for 29/02 with YY = 00 (1900 is not a leap year); future
    // dates are not rejected because the crate has no clock.
    let year = 2000 + number(&chars[date_at..date_at + 2]);
    if !valid_date(year, &chars[date_at..date_at + 6]) {
        return Err(ValidationError::InvalidFormat);
    }
    let body = &chars[..chars.len() - 1];
    let padding = 12usize.saturating_sub(body.len());
    let mut sum = 0u32;
    for i in 0..12 {
        let value = if i < padding {
            rfc_value(b' ')
        } else {
            rfc_value(body[i - padding])
        };
        sum += value * (13 - i as u32);
    }
    let computed = match (11 - sum % 11) % 11 {
        10 => b'A',
        digit => b'0' + digit as u8,
    };
    if chars[chars.len() - 1] != computed {
        return Err(ValidationError::CheckDigitMismatch {
            expected: if computed == b'A' {
                10
            } else {
                (computed - b'0') as u32
            },
            received: if chars[chars.len() - 1] == b'A' {
                10
            } else {
                (chars[chars.len() - 1] - b'0') as u32
            },
        });
    }
    Ok(())
}
