/// Validate a Guatemalan CUI/DPI.
///
/// Layout: correlativo (8) + check digit + departamento (2) + municipio (2).
fn validate_cui(digits: &[u8]) -> Result<(), ValidationError> {
    if digits.len() != 13 {
        return Err(ValidationError::InvalidLength {
            expected: 13,
            found: digits.len(),
        });
    }
    if !digits.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::NonNumeric { index: 0 });
    }
    let mut sum = 0u32;
    for (i, d) in digits[..8].iter().enumerate() {
        sum += (d - b'0') as u32 * (i as u32 + 2);
    }
    let computed = sum % 11;
    let received = (digits[8] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }
    let department = (digits[9] - b'0') as u32 * 10 + (digits[10] - b'0') as u32;
    let municipality = (digits[11] - b'0') as u32 * 10 + (digits[12] - b'0') as u32;
    let Some(&municipalities) = MUNICIPALITIES_PER_DEPARTMENT.get(department as usize) else {
        return Err(ValidationError::InvalidRegion {
            region: department,
        });
    };
    if municipality == 0 || municipality > municipalities as u32 {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Number of municipalities per department, indexed from 01 (Guatemala).
///
/// INE's current list has 340 municipalities; Escuintla gained Sipacate (14)
/// and Huehuetenango gained Petatán (33). Keep this table in sync when a
/// department creates a municipality.
const MUNICIPALITIES_PER_DEPARTMENT: [u8; 23] = [
    0, 17, 8, 16, 16, 14, 14, 19, 8, 24, 21, 9, 30, 33, 21, 8, 17, 14, 5, 11, 11, 7, 17,
];
