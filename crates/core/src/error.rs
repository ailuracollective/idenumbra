//! Shared validation error.

use core::fmt;

/// Reason why an identification was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationError {
    /// The input is empty once surrounding whitespace is trimmed.
    Empty,
    /// The input does not have an accepted length.
    InvalidLength {
        /// Expected length (when fixed).
        expected: usize,
        /// Length actually found.
        found: usize,
    },
    /// The input contains a character that is not an ASCII digit.
    NonNumeric {
        /// Byte index of the first offending character.
        index: usize,
    },
    /// The input contains a character that is not ASCII alphanumeric.
    NonAlphanumeric {
        /// Byte index of the first offending character.
        index: usize,
    },
    /// The country code is not supported by this build.
    UnknownCountry,
    /// A validator for the country exists in source but is not enabled in this build.
    ValidatorNotAvailable {
        /// The country code whose validator is missing.
        country: &'static str,
    },
    /// The check digit does not match the one computed from the body.
    CheckDigitMismatch {
        /// Check digit computed from the body.
        expected: u32,
        /// Check digit found in the input.
        received: u32,
    },
    /// The body produces a check digit that cannot be represented.
    UnrepresentableCheckDigit,
    /// The input does not match the expected format for the document type.
    InvalidFormat,
    /// A province/region prefix is not valid for the country.
    InvalidRegion {
        /// The region code that was read.
        region: u32,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("identification is empty"),
            Self::InvalidLength { expected, found } => {
                write!(f, "expected {expected} characters, found {found}")
            }
            Self::NonNumeric { index } => {
                write!(f, "character at byte {index} is not an ASCII digit")
            }
            Self::NonAlphanumeric { index } => {
                write!(f, "character at byte {index} is not ASCII alphanumeric")
            }
            Self::UnknownCountry => f.write_str("unknown country code"),
            Self::ValidatorNotAvailable { country } => {
                write!(f, "validator for {country} is not available in this build")
            }
            Self::CheckDigitMismatch { expected, received } => {
                write!(
                    f,
                    "check digit mismatch: expected {expected}, received {received}"
                )
            }
            Self::UnrepresentableCheckDigit => {
                f.write_str("body produces an unrepresentable check digit")
            }
            Self::InvalidFormat => f.write_str("invalid identification format"),
            Self::InvalidRegion { region } => write!(f, "invalid region code {region}"),
        }
    }
}

impl core::error::Error for ValidationError {}
