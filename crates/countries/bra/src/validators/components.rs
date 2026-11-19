/// Parts encoded in a validated Brazilian identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// CPF (11 digits).
    Cpf(Cpf),
    /// CNPJ (14 characters).
    Cnpj(Cnpj),
}

/// CPF: region digit, first check digit, second check digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cpf {
    /// Ninth digit, the issuing region.
    pub region: u8,
    /// First check digit (position 10).
    pub check_digit_1: u8,
    /// Second check digit (position 11).
    pub check_digit_2: u8,
}

/// CNPJ: 8-character root, 4-digit branch, two check digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cnpj {
    /// First 8 characters (root), as ASCII bytes.
    pub root: [u8; 8],
    /// Branch number (positions 9-12).
    pub branch: u32,
    /// First check digit (position 13).
    pub check_digit_1: u8,
    /// Second check digit (position 14).
    pub check_digit_2: u8,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"cpf"` or `"cnpj"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32)
}

/// Parts encoded in a validated CPF.
pub fn cpf(digits: &[u8]) -> Cpf {
    Cpf {
        region: digits[8] - b'0',
        check_digit_1: digits[9] - b'0',
        check_digit_2: digits[10] - b'0',
    }
}

/// Parts encoded in a validated CNPJ.
pub fn cnpj(chars: &[u8]) -> Cnpj {
    let mut root = [0u8; 8];
    root.copy_from_slice(&chars[..8]);
    Cnpj {
        root,
        branch: number(&chars[8..12]),
        check_digit_1: chars[12] - b'0',
        check_digit_2: chars[13] - b'0',
    }
}

/// Parts encoded in `value` after validating it.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}