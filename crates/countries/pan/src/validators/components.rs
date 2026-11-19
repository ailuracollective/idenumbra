/// Parts encoded in a validated Panamanian identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// Cédula (hyphenated form). Plain numbers carry no structured components.
    Cedula(Cedula),
}

/// A Panamanian cédula, optionally with the RUC NT flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cedula {
    /// Province code (1-13), or 0 when there is no province (rollo or a
    /// marker-only first part such as "E").
    pub province: u8,
    /// Number of bytes in `marker` that are meaningful (0 when there is none).
    pub marker_len: u8,
    /// Marker bytes (e.g. `PE`, `AV`, `PI`, `E`, `N`), left-aligned and
    /// zero-padded.
    pub marker: [u8; 2],
    /// Folio (middle part).
    pub folio: u32,
    /// Asiento (last part).
    pub asiento: u32,
    /// `true` for the four-part `province-NT-folio-asiento` tax number.
    pub is_nt: bool,
}

/// Result of a successful validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"id_card"`, `"ruc"` or `"passport"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

/// Parse a hyphenated cédula into its components. Returns `None` for plain
/// numbers or passports.
fn parse_cedula(value: &str) -> Option<Cedula> {
    let mut parts = ["", "", "", ""];
    let mut count = 0usize;
    for part in value.split('-') {
        if count == parts.len() {
            return None;
        }
        parts[count] = part;
        count += 1;
    }
    match count {
        3 => {
            let first = parts[0];
            let folio: u32 = parts[1].parse().ok()?;
            let asiento: u32 = parts[2].parse().ok()?;
            let digits = first.bytes().take_while(u8::is_ascii_digit).count();
            let (province, marker_len, marker) = if digits == first.len() {
                (0u8, 0u8, [0u8; 2])
            } else {
                let province = if digits == 0 {
                    0u8
                } else {
                    first[..digits].parse::<u8>().ok()?
                };
                let m = &first[digits..];
                let mut marker = [0u8; 2];
                let mut len = 0u8;
                for (i, b) in m.bytes().enumerate() {
                    if i < 2 {
                        marker[i] = b;
                        len += 1;
                    }
                }
                (province, len, marker)
            };
            Some(Cedula {
                province,
                marker_len,
                marker,
                folio,
                asiento,
                is_nt: false,
            })
        }
        4 => {
            // province-NT-folio-asiento
            let province: u8 = parts[0].parse().ok()?;
            if parts[1] != "NT" {
                return None;
            }
            let folio: u32 = parts[2].parse().ok()?;
            let asiento: u32 = parts[3].parse().ok()?;
            Some(Cedula {
                province,
                marker_len: 0,
                marker: [0u8; 2],
                folio,
                asiento,
                is_nt: true,
            })
        }
        _ => None,
    }
}

/// Parts encoded in `value` after validating it.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}