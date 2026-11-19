/// Validate `value` as an explicit document `kind`; use `validate` to infer.
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError> {
    let value = value.trim();
    let mut buf = [0u8; 12];
    let n = compact(value, &mut buf)?;
    let chars = &buf[..n];
    match kind {
        "id_card" => {
            if n != ID_CARD_LEN {
                return Err(ValidationError::InvalidLength {
                    expected: ID_CARD_LEN,
                    found: n,
                });
            }
            if !chars.iter().all(u8::is_ascii_digit) {
                return Err(ValidationError::InvalidFormat);
            }
            validate_digits(chars)?;
            Ok(Validation {
                kind: "id_card",
                components: Some(Components::CedulaFisica(cedula_fisica(chars))),
            })
        }
        "cedula_juridica" => {
            validate_juridical(chars)?;
            Ok(Validation {
                kind: "cedula_juridica",
                components: Some(Components::CedulaJuridica(cedula_juridica(chars))),
            })
        }
        // Hacienda's *Anexos y Estructuras* v4.4: "El Documento de
        // Identificación de la DGT (NITE) debe contener 10 dígitos y sin
        // guiones". The 12-character field of the municipal reporting form and
        // of the invoice key is that same 10-digit number zero-padded, not a
        // longer identifier.
        "nite" => {
            if n != JURIDICAL_LEN {
                return Err(ValidationError::InvalidLength {
                    expected: JURIDICAL_LEN,
                    found: n,
                });
            }
            if !chars.iter().all(u8::is_ascii_digit) {
                return Err(ValidationError::InvalidFormat);
            }
            // All-zero placeholders are never issued; `validate` rejects them.
            if chars.iter().all(|d| *d == b'0') {
                return Err(ValidationError::InvalidFormat);
            }
            // The annex states no leading-zero rule for the NITE, so a
            // 10-digit value starting with 0 is accepted.
            Ok(Validation {
                kind: "nite",
                components: None,
            })
        }
        "dimex" => {
            if !DIMEX_LENGTHS.contains(&n) {
                return Err(ValidationError::InvalidLength {
                    expected: 12,
                    found: n,
                });
            }
            if !chars.iter().all(u8::is_ascii_digit) {
                return Err(ValidationError::InvalidFormat);
            }
            validate_digits(chars)?;
            Ok(Validation {
                kind: "dimex",
                components: Some(Components::Dimex(dimex(chars))),
            })
        }
        "passport" => {
            validate_passport(chars)?;
            Ok(Validation {
                kind: "passport",
                components: None,
            })
        }
        _ => Err(ValidationError::InvalidFormat),
    }
}