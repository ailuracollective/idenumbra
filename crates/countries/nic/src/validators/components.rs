/// Parts encoded in a validated Nicaraguan cédula.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cedula {
    /// Municipality code (3 digits). Not validated against any official list.
    pub municipality: u32,
    /// Birth date as DDMMYY ASCII bytes.
    pub birth_date: [u8; 6],
    /// Serial number (4 digits).
    pub serial: u32,
    /// Check letter (ASCII, uppercased).
    pub check_letter: u8,
}

/// Parts encoded in a validated Nicaraguan identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// Cédula de identidad (14 characters).
    Cedula(Cedula),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"id_card"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn cedula(chars: &[u8]) -> Cedula {
    let mut birth_date = [0u8; 6];
    birth_date.copy_from_slice(&chars[3..9]);
    Cedula {
        municipality: number(&chars[0..3]),
        birth_date,
        serial: number(&chars[9..13]),
        check_letter: chars[13],
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a passport, which carries no structured components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}