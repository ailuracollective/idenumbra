/// Parts encoded in a validated Bolivian NIT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nit {
    /// The CI embedded in the first `len - 3` digits.
    pub embedded_ci: u64,
    /// The two-digit control code's meaningful digit (`1`-`4`).
    pub control_code: u8,
    /// The final random digit.
    pub trailing: u8,
}

/// Parts encoded in a validated Bolivian identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// NIT (9-11 digits).
    Nit(Nit),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"id_card"` or `"nit"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn number(bytes: &[u8]) -> u64 {
    let mut n = 0u64;
    for b in bytes {
        n = n * 10 + (b - b'0') as u64;
    }
    n
}

fn nit(digits: &[u8]) -> Nit {
    let len = digits.len();
    let code = len - 3;
    Nit {
        embedded_ci: number(&digits[..code]),
        control_code: digits[code + 1] - b'0',
        trailing: digits[len - 1] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a CI, which carries no structured components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}