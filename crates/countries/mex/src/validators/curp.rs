/// CURP state codes: the 31 states plus `DF` and `NE` (born abroad), from the
/// DOF 23/10/1996 Anexo I (`QT` for Querétaro, explicit there). The RENAPO PUI
/// manual of 23/01/2026 prints `QO` and adds `XX` in its API mapping table, but
/// says `XX` is not part of an issued CURP, so neither is accepted here. Edit
/// when a state is created, renamed or split.
const CURP_STATES: [&[u8; 2]; 33] = [
    b"AS", b"BC", b"BS", b"CC", b"CH", b"CL", b"CM", b"CS", b"DF", b"DG", b"GR", b"GT",
    b"HG", b"JC", b"MC", b"MN", b"MS", b"NE", b"NL", b"NT", b"OC", b"PL", b"QR", b"QT",
    b"SL", b"SP", b"SR", b"TC", b"TL", b"TS", b"VZ", b"YN", b"ZS",
];

/// Character values for the CURP checksum: the public table
/// `0123456789ABCDEFGHIJKLMN&OPQRSTUVWXYZ` (index 24 is the Ñ slot; a CURP
/// replaces Ñ with X, so it never occurs). RENAPO describes the check digit in
/// the Instructivo Normativo (DOF 18/10/2021) but does not print the formula.
fn curp_value(b: u8) -> u32 {
    match b {
        b'0'..=b'9' => (b - b'0') as u32,
        b'A'..=b'N' => 10 + (b - b'A') as u32,
        b'&' => 24,
        _ => 25 + (b - b'O') as u32,
    }
}

fn valid_curp_state(state: &[u8]) -> bool {
    CURP_STATES.iter().any(|s| *s == state)
}

/// Positions 14-16 carry the first internal consonant of each name; `X` is the
/// filler when there is none (Instructivo Normativo, DOF 18/10/2021).
fn is_consonant(b: u8) -> bool {
    b"BCDFGHJKLMNPQRSTVWXYZ".contains(&b)
}

/// Validate a Mexican CURP (18 characters, with check digit).
fn validate_curp(chars: &[u8]) -> Result<(), ValidationError> {
    if chars.len() != 18 {
        return Err(ValidationError::InvalidLength {
            expected: 18,
            found: chars.len(),
        });
    }
    if !chars[..4].iter().all(u8::is_ascii_uppercase)
        || !chars[4..10].iter().all(u8::is_ascii_digit)
        || !chars[10..13].iter().all(u8::is_ascii_uppercase)
        || !chars[13..16].iter().all(|c| is_consonant(*c))
        || !chars[16].is_ascii_alphanumeric()
        || !chars[17].is_ascii_digit()
    {
        return Err(ValidationError::InvalidFormat);
    }
    // Position 17 (homonimia + century) is a digit 0-9 up to 1999 and a letter
    // from 2000 on, so its class tells the century (Reglas RENAPO, 22/10/2021).
    let century = if chars[16].is_ascii_digit() { 1900 } else { 2000 };
    let year = century + number(&chars[4..6]);
    if !valid_date(year, &chars[4..10]) {
        return Err(ValidationError::InvalidFormat);
    }
    // RENAPO also issues non-binary CURPs with `X` in this position (the
    // gob.mx form sends "No binario" as `X`); the Instructivo still prints H/M.
    if !matches!(chars[10], b'H' | b'M' | b'X') {
        return Err(ValidationError::InvalidFormat);
    }
    if !valid_curp_state(&chars[11..13]) {
        return Err(ValidationError::InvalidFormat);
    }
    let mut sum = 0u32;
    for (i, &c) in chars[..17].iter().enumerate() {
        sum += curp_value(c) * (18 - i as u32);
    }
    let computed = (10 - sum % 10) % 10;
    let received = (chars[17] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    Ok(())
}
