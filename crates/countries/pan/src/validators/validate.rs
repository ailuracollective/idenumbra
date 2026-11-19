/// Province codes run 1..=MAX_PROVINCE_CODE. Edit when provinces or comarcas
/// are created.
const MAX_PROVINCE_CODE: u32 = 13;

/// Markers written after the province code, or in its place: E extranjero,
/// N naturalizado, PE nacido en el exterior, AV antes de vigencia, PI población
/// indígena. Edit when the Tribunal Electoral issues a new one.
///
/// The old `EE` (foreigner) and `SB` (pre-1953 San Blas) series are absent on
/// purpose: Decreto 53/1958 art. 9, still the numbering rule, lists just
/// province, `PE`, `N` and `E`, and no later norm documents those two.
const ID_MARKERS: [&str; 5] = ["E", "N", "PE", "AV", "PI"];

/// DGI tag for a *Número Tributario*: `province-NT-folio-asiento`, issued to
/// resident foreigners and to entities without legal personality. Edit when
/// the DGI changes its RUC structure.
const NT_TAG: &str = "NT";

/// Panamanian provinces are numbered 01-13.
fn valid_province(digits: &[u8]) -> bool {
    let province = digits
        .iter()
        .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32);
    (1..=MAX_PROVINCE_CODE).contains(&province)
}

/// First part of an ID: a rollo/tomo number, or a province code plus one of
/// `ID_MARKERS`.
fn valid_first_part(part: &str) -> bool {
    if part.is_empty() {
        return false;
    }
    let digits = part.bytes().take_while(u8::is_ascii_digit).count();
    if digits == part.len() {
        let value = part
            .bytes()
            .fold(0u32, |acc, d| acc * 10 + (d - b'0') as u32);
        return value != 0 && digits <= 9;
    }
    let province_ok = digits == 0
        || (digits <= 2 && valid_province(&part.as_bytes()[..digits]));
    province_ok && ID_MARKERS.contains(&&part[digits..])
}

fn all_digits(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit())
}

/// Folio/asiento pair shared by the cédula and RUC shapes: 1-4 digits for the
/// folio and 1-9 digits for the asiento.
fn valid_tail(folio: &str, asiento: &str) -> bool {
    all_digits(folio) && folio.len() <= 4 && all_digits(asiento) && asiento.len() <= 9
}

/// Returns `Ok(true)` when the value is the four-part RUC NT shape
/// (`province-NT-folio-asiento`), `Ok(false)` for the three-part cédula shape.
fn validate_hyphenated(value: &str) -> Result<bool, ValidationError> {
    let mut parts = [""; 4];
    let mut count = 0usize;
    for part in value.split('-') {
        if count == parts.len() {
            return Err(ValidationError::InvalidFormat);
        }
        parts[count] = part;
        count += 1;
    }
    let (ok, is_nt) = match count {
        3 => (valid_first_part(parts[0]) && valid_tail(parts[1], parts[2]), false),
        // RUC NT (DGI): province-NT-folio-asiento.
        4 => (
            parts[1] == NT_TAG
                && all_digits(parts[0])
                && parts[0].len() <= 2
                && valid_province(parts[0].as_bytes())
                && valid_tail(parts[2], parts[3]),
            true,
        ),
        _ => (false, false),
    };
    if ok {
        Ok(is_nt)
    } else {
        Err(ValidationError::InvalidFormat)
    }
}

/// Plain (unhyphenated) number: 7-13 digits, never all zeros.
fn validate_plain(value: &str) -> Result<(), ValidationError> {
    if !all_digits(value) {
        return Err(ValidationError::InvalidFormat);
    }
    if value.bytes().all(|b| b == b'0') {
        return Err(ValidationError::InvalidFormat);
    }
    if (7..=13).contains(&value.len()) {
        Ok(())
    } else {
        Err(ValidationError::InvalidLength {
            expected: 10,
            found: value.len(),
        })
    }
}

fn validate_passport(value: &str) -> Result<(), ValidationError> {
    if !(6..=12).contains(&value.len()) {
        return Err(ValidationError::InvalidLength {
            expected: 6,
            found: value.len(),
        });
    }
    for (i, b) in value.bytes().enumerate() {
        if !b.is_ascii_alphanumeric() {
            return Err(ValidationError::NonAlphanumeric { index: i });
        }
    }
    Ok(())
}

/// Validate a Panamanian identity card: a hyphenated cédula or a plain number.
/// The four-part `province-NT-folio-asiento` tax number is not a cédula.
fn validate_id_card(value: &str) -> Result<(), ValidationError> {
    if value.contains('-') {
        return match validate_hyphenated(value)? {
            true => Err(ValidationError::InvalidFormat),
            false => Ok(()),
        };
    }
    validate_plain(value)
}

/// Validate a Panamanian RUC: a cédula (a natural person's RUC is their cédula),
/// the four-part NT, or a plain number. A passport is not a RUC.
fn validate_ruc(value: &str) -> Result<(), ValidationError> {
    if value.contains('-') {
        return validate_hyphenated(value).map(|_| ());
    }
    validate_plain(value)
}

/// Validate a Panamanian identification number (identity card or RUC).
///
/// The DGI assigns a 2-digit check digit per RUC, and its value depends on the
/// taxpayer type (natural, juridical, natural NT, juridical NT, finca) plus, for
/// natural persons, the province and marker. The DGI publishes the algorithm,
/// but applying it needs the taxpayer type, which this crate does not carry, and
/// it changes when the holder registers as a taxpayer, so it cannot be derived
/// from the number alone. Only the structure is validated here.
pub fn validate(value: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ValidationError::Empty);
    }
    if value.contains('-') {
        let is_nt = validate_hyphenated(value)?;
        let components = parse_cedula(value);
        let kind = if is_nt { "ruc" } else { "id_card" };
        return Ok(Validation {
            kind,
            components: components.map(Components::Cedula),
        });
    }
    if all_digits(value) {
        validate_plain(value)?;
        return Ok(Validation {
            kind: "id_card",
            components: None,
        });
    }
    validate_passport(value)?;
    Ok(Validation {
        kind: "passport",
        components: None,
    })
}
