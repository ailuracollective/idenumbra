/// Parts encoded in a validated Uruguayan identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// Cédula de identidad (7-digit body plus check digit).
    Ci(Ci),
    /// RUT (12 digits).
    Rut(Rut),
}

/// Cédula de identidad: check digit only (the body is a plain sequential number).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ci {
    /// Last digit, modulo 10.
    pub check_digit: u8,
}

/// RUT: operating unit (2), sequential (6), check digit (1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rut {
    /// First two digits: DGI operating unit, `01`-`22`.
    pub operating_unit: u8,
    /// Six sequential digits (positions 3-8).
    pub sequential: u32,
    /// Last digit, modulo 11.
    pub check_digit: u8,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"ci"`, `"rut"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32)
}

fn ci(digits: &[u8]) -> Ci {
    Ci {
        check_digit: digits[digits.len() - 1] - b'0',
    }
}

fn rut(digits: &[u8]) -> Rut {
    Rut {
        operating_unit: (digits[0] - b'0') * 10 + (digits[1] - b'0'),
        sequential: number(&digits[2..8]),
        check_digit: digits[11] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a passport, which carries no structured components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}