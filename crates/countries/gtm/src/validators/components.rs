/// Parts encoded in a validated Guatemalan identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// CUI/DPI (12 or 13 digits).
    Cui(Cui),
    /// NIT.
    Nit(Nit),
}

/// CUI/DPI: check digit, department and municipality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cui {
    /// Check digit (0-9).
    pub check_digit: u8,
    /// Department code (01-22).
    pub department: u8,
    /// Municipality code within the department.
    pub municipality: u8,
}

/// NIT: check character (`0-9`, or `10` for `K`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nit {
    /// Check character encoded as `0-9` or `10` for `K`.
    pub check_char: u8,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"cui"`, `"nit"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

/// Extract the CUI components from 12 or 13 depurated digits (left-padded).
pub fn cui(digits: &[u8]) -> Cui {
    let mut buf = [b'0'; 13];
    buf[13 - digits.len()..].copy_from_slice(digits);
    Cui {
        check_digit: buf[8] - b'0',
        department: (buf[9] - b'0') * 10 + (buf[10] - b'0'),
        municipality: (buf[11] - b'0') * 10 + (buf[12] - b'0'),
    }
}

/// Extract the NIT check character from depurated bytes.
pub fn nit(chars: &[u8]) -> Nit {
    let last = chars[chars.len() - 1].to_ascii_uppercase();
    let check_char = if last == b'K' {
        10
    } else {
        last - b'0'
    };
    Nit { check_char }
}

/// Parts encoded in `value` after validating it.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}