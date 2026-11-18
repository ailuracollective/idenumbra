/// Province names by INEC code (División Política Administrativa). Edit when a
/// province is created, merged or renamed.
const PROVINCES: [&str; 24] = [
    "Azuay",
    "Bolívar",
    "Cañar",
    "Carchi",
    "Cotopaxi",
    "Chimborazo",
    "El Oro",
    "Esmeraldas",
    "Guayas",
    "Imbabura",
    "Loja",
    "Los Ríos",
    "Manabí",
    "Morona Santiago",
    "Napo",
    "Pastaza",
    "Pichincha",
    "Tungurahua",
    "Zamora Chinchipe",
    "Galápagos",
    "Sucumbíos",
    "Orellana",
    "Santo Domingo de los Tsáchilas",
    "Santa Elena",
];

/// Name for series `30`, Ecuadorians registered abroad (Registro Civil).
const ABROAD_NAME: &str = "Ecuatorianos en el exterior";

/// Province encoded in the first two digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Province {
    /// INEC code: `01`-`24`, or `30` for Ecuadorians registered abroad.
    pub code: u32,
    /// INEC province name. `None` for the legacy RUC prefix `50`, which is
    /// not a province.
    pub name: Option<&'static str>,
}

/// Parts encoded in a validated Ecuadorian identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// Cédula de identidad/ciudadanía (10 digits).
    IdCard(IdCard),
    /// RUC (13 digits).
    Ruc(Ruc),
}

/// Cédula: province (2), person type (1), serial (6), check digit (1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdCard {
    /// First two digits.
    pub province: Province,
    /// Third digit, `0`-`5`. The Registro Civil publishes no finer meaning,
    /// so it is returned raw.
    pub person_type: u8,
    /// Digits 4-9.
    pub serial: u32,
    /// Last digit, modulo 10.
    pub check_digit: u8,
}

/// Taxpayer type encoded in the third RUC digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Taxpayer {
    /// `0`-`5`: natural person; the RUC is a cédula followed by `001`.
    NaturalPerson,
    /// `6`: public entity.
    PublicEntity,
    /// `9`: private company or foreign person without a cédula.
    PrivateCompany,
}

/// RUC: province (2), taxpayer type (1), then a type-dependent body and `001`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ruc {
    /// First two digits.
    pub province: Province,
    /// Third digit.
    pub taxpayer: Taxpayer,
    /// Natural persons only: the cédula in the first ten digits.
    pub id_card: Option<IdCard>,
    /// Public entities only: the five sequential digits after the taxpayer
    /// digit. Private-company sequentials are not exposed because they may be
    /// six or seven digits and cannot be split from the check digit.
    pub serial: Option<u32>,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"id_card"`, `"ruc"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn province(code: u32) -> Province {
    let name = match code {
        1..=MAX_PROVINCE_CODE => Some(PROVINCES[code as usize - 1]),
        ABROAD_PROVINCE_CODE => Some(ABROAD_NAME),
        _ => None,
    };
    Province { code, name }
}

fn id_card(digits: &[u8]) -> IdCard {
    IdCard {
        province: province(number(&digits[0..2])),
        person_type: digits[2] - b'0',
        serial: number(&digits[3..9]),
        check_digit: digits[9] - b'0',
    }
}

fn ruc(digits: &[u8]) -> Ruc {
    let taxpayer = match digits[2] {
        b'6' => Taxpayer::PublicEntity,
        b'9' => Taxpayer::PrivateCompany,
        _ => Taxpayer::NaturalPerson,
    };
    Ruc {
        province: province(number(&digits[0..2])),
        taxpayer,
        id_card: match taxpayer {
            Taxpayer::NaturalPerson => Some(id_card(&digits[..10])),
            _ => None,
        },
        serial: match taxpayer {
            Taxpayer::PublicEntity => Some(number(&digits[3..8])),
            _ => None,
        },
    }
}

/// Parts encoded in `value` after validating it.
///
/// Returns `Ok(None)` for a passport, which carries no structured components.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}
