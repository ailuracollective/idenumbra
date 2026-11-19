/// Parts encoded in a validated Honduran RTN.
///
/// A natural person's RTN is the 13-digit DNI followed by a trailing digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rtn {
    /// First 13 digits: the embedded DNI.
    pub embedded_dni: u64,
    /// Fourteenth digit.
    pub trailing: u8,
}

/// Parts encoded in a validated Honduran identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// RTN (14 digits).
    Rtn(Rtn),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"dni"`, `"rtn"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn digits_to_u64(digits: &[u8]) -> u64 {
    digits.iter().fold(0u64, |acc, d| acc * 10 + (d - b'0') as u64)
}

fn rtn_from_digits(digits: &[u8]) -> Rtn {
    Rtn {
        embedded_dni: digits_to_u64(&digits[..13]),
        trailing: digits[13] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a DNI or passport, which carry no structured
/// components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}