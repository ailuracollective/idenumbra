/// Parts encoded in a validated Mexican identification number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Components {
    /// CURP (18 characters).
    Curp(Curp),
    /// RFC (12 characters for companies, 13 for persons).
    Rfc(Rfc),
}

/// CURP: name code (4), birth date (6, YYMMDD), sex (1), state (2),
/// internal consonants (3), century differentiator (1), check digit (1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Curp {
    /// First four characters: surname/name initials.
    pub name_code: [u8; 4],
    /// Birth date as `YYMMDD` bytes.
    pub birth_date: [u8; 6],
    /// Sex: `H`, `M` or `X`.
    pub sex: u8,
    /// Two-letter state code from the RENAPO Anexo I table.
    pub state: [u8; 2],
    /// First internal consonant of each name part.
    pub internal_consonants: [u8; 3],
    /// Century differentiator: `0`-`9` up to 1999, `A`-`J` from 2000.
    pub century_marker: u8,
    /// Check digit (0-9).
    pub check_digit: u8,
}

/// RFC: name letters (3 for companies, 4 for persons), birth date (6, YYMMDD),
/// homoclave and check character (last position; `10` is written as `A`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rfc {
    /// Name letters, left-aligned; unused slots are zero.
    pub initials: [u8; 4],
    /// Number of letters actually present (3 for companies, 4 for persons).
    pub initials_len: u8,
    /// Birth date as `YYMMDD` bytes.
    pub birth_date: [u8; 6],
    /// Last character: check digit (`0`-`9`) or `A` for a computed 10.
    pub check_char: u8,
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its [`Components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Validated document kind: `"curp"` or `"rfc"`.
    pub kind: &'static str,
    /// Parts encoded in the number, if any.
    pub components: Option<Components>,
}

fn curp(chars: &[u8]) -> Curp {
    Curp {
        name_code: [chars[0], chars[1], chars[2], chars[3]],
        birth_date: [chars[4], chars[5], chars[6], chars[7], chars[8], chars[9]],
        sex: chars[10],
        state: [chars[11], chars[12]],
        internal_consonants: [chars[13], chars[14], chars[15]],
        century_marker: chars[16],
        check_digit: chars[17] - b'0',
    }
}

fn rfc(chars: &[u8]) -> Rfc {
    let letters = chars.len() - 9;
    let date_at = letters;
    let mut initials = [0u8; 4];
    initials[..letters].copy_from_slice(&chars[..letters]);
    Rfc {
        initials,
        initials_len: letters as u8,
        birth_date: [
            chars[date_at],
            chars[date_at + 1],
            chars[date_at + 2],
            chars[date_at + 3],
            chars[date_at + 4],
            chars[date_at + 5],
        ],
        check_char: {
            let last = chars[chars.len() - 1];
            if last == b'A' {
                10
            } else {
                last - b'0'
            }
        },
    }
}

/// Parts encoded in `value` after validating it.
pub fn components(value: &str) -> Result<Option<Components>, ValidationError> {
    validate(value).map(|v| v.components)
}
