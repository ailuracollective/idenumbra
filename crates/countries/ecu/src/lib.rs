//! Ecuador identification validator.
//!
//! Validates ID card (10 digits), RUC (13 digits) and passport (alphanumeric)
//! numbers. Implements [`idenumbra_core::Validator`], so it can be used on its
//! own or registered in the orchestrator. Validated numbers can also be split
//! into their parts with [`components`].

#![no_std]

use idenumbra_core::{ValidationError, Validator};

const COUNTRY: &str = "ECU";

mod validators {
    use idenumbra_core::ValidationError;
    include!("validators/validate.rs");
    include!("validators/id_card.rs");
    include!("validators/passport.rs");
    include!("validators/ruc.rs");
    include!("validators/validate_with.rs");
    include!("validators/components.rs");
}

pub use validators::{
    components, validate, validate_with, Components, IdCard, Province, Ruc, Taxpayer, Validation,
};

/// Zero-sized marker type implementing [`Validator`] for Ecuador.
pub struct IdenumbraECU;

impl Validator for IdenumbraECU {
    const COUNTRY: &'static str = COUNTRY;
    type Validation = Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError> {
        validate(value)
    }
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> {
        crate::validate_with(value, kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_id_card_and_rucs() {
        assert!(validate("1712345675").is_ok());
        assert!(validate("1712345675001").is_ok());
        assert!(validate("1790012344001").is_ok());
        assert!(validate("1760001200001").is_ok());
        assert!(validate("1790012345001").is_ok());
        assert!(validate("5090012340001").is_ok());
    }

    #[test]
    fn rejects_bad_check_digits() {
        assert_eq!(
            validate("1712345676"),
            Err(ValidationError::CheckDigitMismatch {
                expected: 5,
                received: 6
            })
        );
        assert!(validate("1760001210001").is_err());
    }

    #[test]
    fn rejects_bad_structure() {
        assert_eq!(
            validate("2512345675"),
            Err(ValidationError::InvalidRegion { region: 25 })
        );
        assert_eq!(
            validate("5012345675"),
            Err(ValidationError::InvalidRegion { region: 50 })
        );
        assert!(validate("1712345675002").is_err());
        assert!(validate("1790012344002").is_err());
        assert!(validate("1760001200002").is_err());
        assert!(validate("1770012340001").is_err());
        assert!(validate("17123458").is_err());
    }

    #[test]
    fn accepts_hyphen_before_last_digit() {
        assert!(validate("171234567-5").is_ok());
        assert!(validate("171234567500-1").is_ok());
        assert!(validate_with("171234567-5", "id_card").is_ok());
        assert!(validate_with("171234567500-1", "ruc").is_ok());
        assert!(validate("171234-5675").is_err());
        assert!(validate("17-12345675").is_err());
        assert!(validate_with("1712345-675", "id_card").is_err());
    }

    #[test]
    fn kinds() {
        assert!(validate_with("1712345675", "id_card").is_ok());
        assert!(validate_with("1712345675001", "ruc").is_ok());
        assert!(validate_with("1712345675", "ruc").is_err());
        assert!(validate_with("A123456", "passport").is_ok());
    }

    fn validated(value: &str) -> Validation {
        match validate(value) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn components_of_id_card() {
        let v = validated("1712345675");
        assert_eq!(v.kind, "id_card");
        let card = match v.components {
            Some(Components::IdCard(c)) => c,
            _ => panic!("expected id card"),
        };
        assert_eq!(card.province.code, 17);
        assert_eq!(card.province.name, Some("Pichincha"));
        assert_eq!(card.person_type, 1);
        assert_eq!(card.serial, 234567);
        assert_eq!(card.check_digit, 5);
    }

    #[test]
    fn components_of_ruc() {
        let v = validated("1712345675001");
        assert_eq!(v.kind, "ruc");
        let ruc = match v.components {
            Some(Components::Ruc(r)) => r,
            _ => panic!("expected ruc"),
        };
        assert_eq!(ruc.taxpayer, Taxpayer::NaturalPerson);
        let embedded = ruc.id_card.unwrap_or_else(|| panic!("expected id card"));
        assert_eq!(embedded.province.code, 17);

        let v = validated("1760001200001");
        let ruc = match v.components {
            Some(Components::Ruc(r)) => r,
            _ => panic!("expected ruc"),
        };
        assert_eq!(ruc.taxpayer, Taxpayer::PublicEntity);
        assert_eq!(ruc.serial, Some(12));

        let v = validated("1790012344001");
        let ruc = match v.components {
            Some(Components::Ruc(r)) => r,
            _ => panic!("expected ruc"),
        };
        assert_eq!(ruc.taxpayer, Taxpayer::PrivateCompany);
        assert_eq!(ruc.serial, None);

        let v = validated("5090012340001");
        let ruc = match v.components {
            Some(Components::Ruc(r)) => r,
            _ => panic!("expected ruc"),
        };
        assert_eq!(ruc.province.code, 50);
        assert_eq!(ruc.province.name, None);
    }

    #[test]
    fn components_require_a_valid_number() {
        assert!(validate("1712345676").is_err());
        let v = validated("A123456");
        assert_eq!(v.kind, "passport");
        assert_eq!(v.components, None);
    }
}
