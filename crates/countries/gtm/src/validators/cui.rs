/// Validate a Guatemalan CUI/DPI.
///
/// Layout: correlativo (8) + check digit + departamento (2) + municipio (2).
/// SAT's FEL annex (`Reglas y validaciones` 2.1, §5.1.4) documents the check
/// digit: the seven or eight correlativo digits are multiplied by weights from
/// 9 down to 2, the products are summed, the sum is multiplied by 10 and taken
/// mod 11. That is the same rule as `sum(first 8 digits × 2..9) mod 11`; when
/// it yields 10 there is no digit to print, so such bodies are unrepresentable.
fn validate_cui(digits: &[u8]) -> Result<(), ValidationError> {
    // §5.1.4.1: a CUI has 12 or 13 digits; the 12-digit form is a correlativo
    // that lost its leading zero, so left-padding recreates the 13-digit layout.
    if !matches!(digits.len(), 12 | 13) {
        return Err(ValidationError::InvalidLength {
            expected: 13,
            found: digits.len(),
        });
    }
    if let Some(index) = digits.iter().position(|b| !b.is_ascii_digit()) {
        return Err(ValidationError::NonNumeric { index });
    }
    let mut cui = [b'0'; 13];
    cui[13 - digits.len()..].copy_from_slice(digits);

    let mut sum = 0u32;
    for (i, d) in cui[..8].iter().enumerate() {
        sum += (d - b'0') as u32 * (i as u32 + 2);
    }
    let computed = sum % 11;
    if computed == 10 {
        return Err(ValidationError::UnrepresentableCheckDigit);
    }
    let received = (cui[8] - b'0') as u32;
    if computed != received {
        return Err(ValidationError::CheckDigitMismatch {
            expected: computed,
            received,
        });
    }

    // §5.1.5: the last four digits are the department and then the municipality,
    // both 01-based and without gaps.
    let department = (cui[9] - b'0') as u32 * 10 + (cui[10] - b'0') as u32;
    let municipality = (cui[11] - b'0') as u32 * 10 + (cui[12] - b'0') as u32;
    let Some(&municipalities) = MUNICIPALITIES_PER_DEPARTMENT.get(department as usize) else {
        return Err(ValidationError::InvalidRegion {
            region: department,
        });
    };
    // Index 0 of the table is unused, so department 00 lands here.
    if municipalities == 0 {
        return Err(ValidationError::InvalidRegion {
            region: department,
        });
    }
    if municipality == 0 || municipality > municipalities as u32 {
        return Err(ValidationError::InvalidFormat);
    }
    Ok(())
}

/// Number of municipalities per department, indexed from 01 (Guatemala).
///
/// INE's current list has 340 municipalities; Escuintla gained Sipacate (14,
/// Decreto 4-2015) and Huehuetenango gained Petatán (33, Decreto 6-2015). SAT's
/// annex still prints 32 for Huehuetenango in its 2.1 edition (October 2026),
/// but INE and the IGN boundary layer assign code 1333. Keep this table in sync
/// when a department creates a municipality.
const MUNICIPALITIES_PER_DEPARTMENT: [u8; 23] = [
    0, 17, 8, 16, 16, 14, 14, 19, 8, 24, 21, 9, 30, 33, 21, 8, 17, 14, 5, 11, 11, 7, 17,
];
