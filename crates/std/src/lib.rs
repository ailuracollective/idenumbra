//! Orchestrator for idenumbra country validators.

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Orchestrator for country identification validators.
pub struct Idenumbra;

/// Country codes compiled into this build.
fn supported_countries() -> &'static [&'static str] {
    &[
        #[cfg(feature = "arg")]
        "ARG",
        #[cfg(feature = "bol")]
        "BOL",
        #[cfg(feature = "bra")]
        "BRA",
        #[cfg(feature = "chl")]
        "CHL",
        #[cfg(feature = "col")]
        "COL",
        #[cfg(feature = "cri")]
        "CRI",
        #[cfg(feature = "cub")]
        "CUB",
        #[cfg(feature = "dom")]
        "DOM",
        #[cfg(feature = "ecu")]
        "ECU",
        #[cfg(feature = "gtm")]
        "GTM",
        #[cfg(feature = "hnd")]
        "HND",
        #[cfg(feature = "mex")]
        "MEX",
        #[cfg(feature = "nic")]
        "NIC",
        #[cfg(feature = "pan")]
        "PAN",
        #[cfg(feature = "per")]
        "PER",
        #[cfg(feature = "pry")]
        "PRY",
        #[cfg(feature = "slv")]
        "SLV",
        #[cfg(feature = "ury")]
        "URY",
        #[cfg(feature = "ven")]
        "VEN",
    ]
}

/// Untyped validator: infers the document kind from the value.
type InferFn = fn(&str) -> Result<(), ValidationError>;

/// Typed validator: checks the value as the given document kind.
type TypedFn = fn(&str, &str) -> Result<(), ValidationError>;

static REGISTRY: OnceLock<BTreeMap<&'static str, (InferFn, TypedFn)>> = OnceLock::new();

fn registry() -> &'static BTreeMap<&'static str, (InferFn, TypedFn)> {
    REGISTRY.get_or_init(|| {
        #[allow(unused_mut)]
        let mut m: BTreeMap<&'static str, (InferFn, TypedFn)> = BTreeMap::new();
        #[cfg(feature = "arg")]
        {
            m.insert(
                "ARG",
                (idenumbra_arg::validate, idenumbra_arg::validate_with),
            );
        }
        #[cfg(feature = "bol")]
        {
            m.insert(
                "BOL",
                (idenumbra_bol::validate, idenumbra_bol::validate_with),
            );
        }
        #[cfg(feature = "bra")]
        {
            m.insert(
                "BRA",
                (idenumbra_bra::validate, idenumbra_bra::validate_with),
            );
        }
        #[cfg(feature = "chl")]
        {
            m.insert(
                "CHL",
                (idenumbra_chl::validate, idenumbra_chl::validate_with),
            );
        }
        #[cfg(feature = "col")]
        {
            m.insert(
                "COL",
                (idenumbra_col::validate, idenumbra_col::validate_with),
            );
        }
        #[cfg(feature = "cri")]
        {
            m.insert(
                "CRI",
                (idenumbra_cri::validate, idenumbra_cri::validate_with),
            );
        }
        #[cfg(feature = "cub")]
        {
            m.insert(
                "CUB",
                (idenumbra_cub::validate, idenumbra_cub::validate_with),
            );
        }
        #[cfg(feature = "dom")]
        {
            m.insert(
                "DOM",
                (idenumbra_dom::validate, idenumbra_dom::validate_with),
            );
        }
        #[cfg(feature = "ecu")]
        {
            m.insert(
                "ECU",
                (idenumbra_ecu::validate, idenumbra_ecu::validate_with),
            );
        }
        #[cfg(feature = "gtm")]
        {
            m.insert(
                "GTM",
                (idenumbra_gtm::validate, idenumbra_gtm::validate_with),
            );
        }
        #[cfg(feature = "hnd")]
        {
            m.insert(
                "HND",
                (idenumbra_hnd::validate, idenumbra_hnd::validate_with),
            );
        }
        #[cfg(feature = "mex")]
        {
            m.insert(
                "MEX",
                (idenumbra_mex::validate, idenumbra_mex::validate_with),
            );
        }
        #[cfg(feature = "nic")]
        {
            m.insert(
                "NIC",
                (idenumbra_nic::validate, idenumbra_nic::validate_with),
            );
        }
        #[cfg(feature = "pan")]
        {
            m.insert(
                "PAN",
                (idenumbra_pan::validate, idenumbra_pan::validate_with),
            );
        }
        #[cfg(feature = "per")]
        {
            m.insert(
                "PER",
                (idenumbra_per::validate, idenumbra_per::validate_with),
            );
        }
        #[cfg(feature = "pry")]
        {
            m.insert(
                "PRY",
                (idenumbra_pry::validate, idenumbra_pry::validate_with),
            );
        }
        #[cfg(feature = "slv")]
        {
            m.insert(
                "SLV",
                (idenumbra_slv::validate, idenumbra_slv::validate_with),
            );
        }
        #[cfg(feature = "ury")]
        {
            m.insert(
                "URY",
                (idenumbra_ury::validate, idenumbra_ury::validate_with),
            );
        }
        #[cfg(feature = "ven")]
        {
            m.insert(
                "VEN",
                (idenumbra_ven::validate, idenumbra_ven::validate_with),
            );
        }
        m
    })
}

/// Resolve a country code to its compiled-in validators.
fn lookup(country: &str) -> Result<&'static (InferFn, TypedFn), ValidationError> {
    let code = country.trim().to_ascii_uppercase();
    let Some(code) = supported_countries()
        .iter()
        .copied()
        .find(|c| *c == code.as_str())
    else {
        return match idenumbra_core::country::canonical(code.as_str()) {
            Some(c) => Err(ValidationError::ValidatorNotAvailable { country: c }),
            None => Err(ValidationError::UnknownCountry),
        };
    };
    registry()
        .get(code)
        .ok_or(ValidationError::ValidatorNotAvailable { country: code })
}

impl Idenumbra {
    /// Country codes compiled into this build.
    pub fn supported_countries() -> &'static [&'static str] {
        supported_countries()
    }

    /// Returns true when a validator for country is compiled in.
    pub fn is_available(country: &str) -> bool {
        let code = country.trim().to_ascii_uppercase();
        supported_countries().contains(&code.as_str())
    }

    /// Validate `value` as an explicit document `kind` (e.g. `"id_card"`,
    /// `"ruc"`, `"passport"`), skipping type inference.
    pub fn validate(country: &str, value: &str, kind: &str) -> Result<(), ValidationError> {
        lookup(country)?.1(value, kind)
    }

    /// Validate `value` letting the country validator infer the document kind.
    pub fn infer(country: &str, value: &str) -> Result<(), ValidationError> {
        lookup(country)?.0(value)
    }
}

pub use idenumbra_core::{ValidationError, Validator};

#[cfg(feature = "arg")]
pub use idenumbra_arg::IdenumbraARG;
#[cfg(feature = "bol")]
pub use idenumbra_bol::IdenumbraBOL;
#[cfg(feature = "bra")]
pub use idenumbra_bra::IdenumbraBRA;
#[cfg(feature = "chl")]
pub use idenumbra_chl::IdenumbraCHL;
#[cfg(feature = "col")]
pub use idenumbra_col::IdenumbraCOL;
#[cfg(feature = "cri")]
pub use idenumbra_cri::IdenumbraCRI;
#[cfg(feature = "cub")]
pub use idenumbra_cub::IdenumbraCUB;
#[cfg(feature = "dom")]
pub use idenumbra_dom::IdenumbraDOM;
#[cfg(feature = "ecu")]
pub use idenumbra_ecu::IdenumbraECU;
#[cfg(feature = "gtm")]
pub use idenumbra_gtm::IdenumbraGTM;
#[cfg(feature = "hnd")]
pub use idenumbra_hnd::IdenumbraHND;
#[cfg(feature = "mex")]
pub use idenumbra_mex::IdenumbraMEX;
#[cfg(feature = "nic")]
pub use idenumbra_nic::IdenumbraNIC;
#[cfg(feature = "pan")]
pub use idenumbra_pan::IdenumbraPAN;
#[cfg(feature = "per")]
pub use idenumbra_per::IdenumbraPER;
#[cfg(feature = "pry")]
pub use idenumbra_pry::IdenumbraPRY;
#[cfg(feature = "slv")]
pub use idenumbra_slv::IdenumbraSLV;
#[cfg(feature = "ury")]
pub use idenumbra_ury::IdenumbraURY;
#[cfg(feature = "ven")]
pub use idenumbra_ven::IdenumbraVEN;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(feature = "ecu")]
    fn validate_ecu() {
        assert!(Idenumbra::infer("ecu", "1712345675").is_ok());
        assert!(Idenumbra::infer("ECU", "1712345676").is_err());
    }
    #[test]
    fn unknown_country() {
        assert_eq!(
            Idenumbra::infer("ZZZ", "123"),
            Err(ValidationError::UnknownCountry)
        );
    }
    #[cfg(feature = "ecu")]
    #[test]
    fn validate_ecu_typed() {
        assert!(Idenumbra::validate("ECU", "1712345675", "id_card").is_ok());
        assert!(Idenumbra::validate("ECU", "1712345675001", "ruc").is_ok());
        assert!(Idenumbra::validate("ECU", "1712345675", "ruc").is_err());
        assert!(Idenumbra::validate("ECU", "171234567-5", "id_card").is_ok());
    }
    #[cfg(feature = "arg")]
    #[test]
    fn validate_arg_typed() {
        assert!(Idenumbra::validate("ARG", "12345678", "dni").is_ok());
        assert!(Idenumbra::validate("ARG", "20-12345678-6", "cuit").is_ok());
        assert!(Idenumbra::validate("ARG", "12345678", "cuit").is_err());
    }

    #[test]
    fn supported_countries_matches_enabled_features() {
        let sc = Idenumbra::supported_countries();
        #[cfg(feature = "arg")]
        assert!(sc.contains(&"ARG"));
        #[cfg(not(feature = "arg"))]
        assert!(!sc.contains(&"ARG"));
        #[cfg(feature = "bol")]
        assert!(sc.contains(&"BOL"));
        #[cfg(not(feature = "bol"))]
        assert!(!sc.contains(&"BOL"));
        #[cfg(feature = "bra")]
        assert!(sc.contains(&"BRA"));
        #[cfg(not(feature = "bra"))]
        assert!(!sc.contains(&"BRA"));
        #[cfg(feature = "chl")]
        assert!(sc.contains(&"CHL"));
        #[cfg(not(feature = "chl"))]
        assert!(!sc.contains(&"CHL"));
        #[cfg(feature = "col")]
        assert!(sc.contains(&"COL"));
        #[cfg(not(feature = "col"))]
        assert!(!sc.contains(&"COL"));
        #[cfg(feature = "cri")]
        assert!(sc.contains(&"CRI"));
        #[cfg(not(feature = "cri"))]
        assert!(!sc.contains(&"CRI"));
        #[cfg(feature = "cub")]
        assert!(sc.contains(&"CUB"));
        #[cfg(not(feature = "cub"))]
        assert!(!sc.contains(&"CUB"));
        #[cfg(feature = "dom")]
        assert!(sc.contains(&"DOM"));
        #[cfg(not(feature = "dom"))]
        assert!(!sc.contains(&"DOM"));
        #[cfg(feature = "ecu")]
        assert!(sc.contains(&"ECU"));
        #[cfg(not(feature = "ecu"))]
        assert!(!sc.contains(&"ECU"));
        #[cfg(feature = "gtm")]
        assert!(sc.contains(&"GTM"));
        #[cfg(not(feature = "gtm"))]
        assert!(!sc.contains(&"GTM"));
        #[cfg(feature = "hnd")]
        assert!(sc.contains(&"HND"));
        #[cfg(not(feature = "hnd"))]
        assert!(!sc.contains(&"HND"));
        #[cfg(feature = "mex")]
        assert!(sc.contains(&"MEX"));
        #[cfg(not(feature = "mex"))]
        assert!(!sc.contains(&"MEX"));
        #[cfg(feature = "nic")]
        assert!(sc.contains(&"NIC"));
        #[cfg(not(feature = "nic"))]
        assert!(!sc.contains(&"NIC"));
        #[cfg(feature = "pan")]
        assert!(sc.contains(&"PAN"));
        #[cfg(not(feature = "pan"))]
        assert!(!sc.contains(&"PAN"));
        #[cfg(feature = "per")]
        assert!(sc.contains(&"PER"));
        #[cfg(not(feature = "per"))]
        assert!(!sc.contains(&"PER"));
        #[cfg(feature = "pry")]
        assert!(sc.contains(&"PRY"));
        #[cfg(not(feature = "pry"))]
        assert!(!sc.contains(&"PRY"));
        #[cfg(feature = "slv")]
        assert!(sc.contains(&"SLV"));
        #[cfg(not(feature = "slv"))]
        assert!(!sc.contains(&"SLV"));
        #[cfg(feature = "ury")]
        assert!(sc.contains(&"URY"));
        #[cfg(not(feature = "ury"))]
        assert!(!sc.contains(&"URY"));
        #[cfg(feature = "ven")]
        assert!(sc.contains(&"VEN"));
        #[cfg(not(feature = "ven"))]
        assert!(!sc.contains(&"VEN"));

        let expected = 0usize
            + cfg!(feature = "arg") as usize
            + cfg!(feature = "bol") as usize
            + cfg!(feature = "bra") as usize
            + cfg!(feature = "chl") as usize
            + cfg!(feature = "col") as usize
            + cfg!(feature = "cri") as usize
            + cfg!(feature = "cub") as usize
            + cfg!(feature = "dom") as usize
            + cfg!(feature = "ecu") as usize
            + cfg!(feature = "gtm") as usize
            + cfg!(feature = "hnd") as usize
            + cfg!(feature = "mex") as usize
            + cfg!(feature = "nic") as usize
            + cfg!(feature = "pan") as usize
            + cfg!(feature = "per") as usize
            + cfg!(feature = "pry") as usize
            + cfg!(feature = "slv") as usize
            + cfg!(feature = "ury") as usize
            + cfg!(feature = "ven") as usize;
        assert_eq!(sc.len(), expected);
    }

    #[test]
    fn supported_countries_is_cached() {
        let a = Idenumbra::supported_countries();
        let b = Idenumbra::supported_countries();
        assert_eq!(a.as_ptr(), b.as_ptr());
    }

    #[cfg(not(feature = "ven"))]
    #[test]
    fn disabled_country_is_not_available() {
        assert!(!Idenumbra::is_available("VEN"));
        assert_eq!(
            Idenumbra::infer("VEN", "12345678"),
            Err(ValidationError::ValidatorNotAvailable { country: "VEN" })
        );
    }
}
