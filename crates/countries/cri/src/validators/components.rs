/// Parts encoded in a validated Costa Rican identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// Cédula de identidad física (9 digits).
    CedulaFisica(CedulaFisica),
    /// Cédula jurídica (10 characters).
    CedulaJuridica(CedulaJuridica),
    /// DIMEX (11 or 12 digits).
    Dimex(Dimex),
}

/// Cédula física: the first digit is the province code (`1`-`9`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CedulaFisica {
    /// First digit of the number.
    pub province: u8,
}

/// Cédula jurídica: a person-type digit followed by a three-digit class code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CedulaJuridica {
    /// First digit (`2`-`5` in practice).
    pub person_type: u8,
    /// Digits 2-4.
    pub class: u32,
}

/// DIMEX: the first three digits are the issuing country code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimex {
    /// First three digits.
    pub country_code: u32,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"id_card"`, `"cedula_juridica"`, `"nite"`,
    /// `"dimex"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn number(digits: &[u8]) -> u32 {
    let mut n = 0u32;
    for &b in digits {
        n = n * 10 + (b - b'0') as u32;
    }
    n
}

fn cedula_fisica(chars: &[u8]) -> CedulaFisica {
    CedulaFisica {
        province: chars[0] - b'0',
    }
}

fn cedula_juridica(chars: &[u8]) -> CedulaJuridica {
    CedulaJuridica {
        person_type: chars[0] - b'0',
        class: number(&chars[1..4]),
    }
}

fn dimex(chars: &[u8]) -> Dimex {
    Dimex {
        country_code: number(&chars[0..3]),
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a NITE or a passport, which carry no structured
/// components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}