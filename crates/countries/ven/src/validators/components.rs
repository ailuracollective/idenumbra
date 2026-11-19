/// Parts encoded in a validated Venezuelan RIF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rif {
    /// RIF letter: `V`, `E`, `J`, `P`, `G` or `C` (ASCII byte).
    pub letter: u8,
    /// The eight body digits.
    pub embedded_id: u32,
    /// Last digit, modulo 11.
    pub check_digit: u8,
}

/// Parts encoded in a validated Venezuelan identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// RIF (letter + 8 digits + check digit).
    Rif(Rif),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"ci"`, `"rif"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn number(bytes: &[u8]) -> u32 {
    let mut n = 0u32;
    for b in bytes {
        n = n * 10 + (b - b'0') as u32;
    }
    n
}

fn rif(chars: &[u8]) -> Rif {
    Rif {
        letter: chars[0],
        embedded_id: number(&chars[1..9]),
        check_digit: chars[9] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a CI or passport, which carry no structured
/// components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}