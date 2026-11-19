/// Parts encoded in a validated Peruvian RUC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ruc {
    /// Taxpayer-type prefix: `10` (natural person with DNI), `15` (foreign
    /// natural person), `17` (implementation-only) or `20` (legal person).
    pub prefix: u8,
    /// The DNI embedded in positions 3-10 when `prefix == 10`, otherwise
    /// `None`.
    pub embedded_dni: Option<u32>,
    /// Last digit (SUNAT's "dígito verificador").
    pub check_digit: u8,
}

/// Parts encoded in a validated Peruvian identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// RUC (11 digits).
    Ruc(Ruc),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"dni"`, `"ruc"`, `"passport"` or `"ce"`.
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

fn ruc(digits: &[u8]) -> Ruc {
    let prefix = (digits[0] - b'0') * 10 + (digits[1] - b'0');
    let embedded_dni = if prefix == 10 {
        Some(number(&digits[2..10]))
    } else {
        None
    };
    Ruc {
        prefix,
        embedded_dni,
        check_digit: digits[10] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a DNI or passport/carné, which carry no structured
/// components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}
