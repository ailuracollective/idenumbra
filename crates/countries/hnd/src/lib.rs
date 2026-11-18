//! Honduras identification validator.

#![no_std]

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/rtn.rs");
    include!("validators/validate_with.rs");
}

pub use validators::{validate, validate_with};

idenumbra_core::country_validator!(IdenumbraHND, "HND");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_dni_and_rtn() {
        assert!(validate("1234567890123").is_ok());
        assert!(validate("12345678901230").is_ok());
        assert!(validate("1234-5678-901230").is_ok());
        assert!(validate_with("1234567890123", "dni").is_ok());
        assert!(validate_with("12345678901230", "rtn").is_ok());
    }

    #[test]
    fn checks_structure_only() {
        // The SAR does not publish the RTN verifier algorithm: the last digit
        // is not checked.
        assert!(validate("12345678901231").is_ok());
        assert!(validate("12345678901239").is_ok());
        assert!(validate("123456789012").is_err());
        assert!(validate("0000000000000").is_err());
        assert!(validate("00000000000000").is_err());
        assert!(validate_with("1234567890123", "rtn").is_err());
        assert!(validate_with("12345678901230", "dni").is_err());
    }
}
