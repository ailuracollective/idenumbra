/// Parts encoded in a validated Argentine CUIT/CUIL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cuit {
    /// First two digits (prefix).
    pub prefix: u8,
    /// Middle eight digits (DNI).
    pub dni: u32,
    /// Last digit (check digit).
    pub check_digit: u8,
}

/// Parts encoded in a validated Argentine identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// CUIT/CUIL (11 digits).
    Cuit(Cuit),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"dni"`, `"cuit"` or `"cuil"`.
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

fn cuit(digits: &[u8]) -> Cuit {
    Cuit {
        prefix: (digits[0] - b'0') * 10 + (digits[1] - b'0'),
        dni: number(&digits[2..10]),
        check_digit: digits[10] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a DNI, which carries no structured components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}