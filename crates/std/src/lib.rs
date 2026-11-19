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

/// Components parser: validates the value and returns the country's parts.
type ComponentsFn = fn(&str) -> Result<Option<CountryComponents>, ValidationError>;

/// Country-specific parts returned by [`Idenumbra::components`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountryComponents {
    /// Argentina: see [`idenumbra_arg::components`].
    #[cfg(feature = "arg")]
    ARG(idenumbra_arg::Components),
    /// Bolivia: see [`idenumbra_bol::components`].
    #[cfg(feature = "bol")]
    BOL(idenumbra_bol::Components),
    /// Brazil: see [`idenumbra_bra::components`].
    #[cfg(feature = "bra")]
    BRA(idenumbra_bra::Components),
    /// Costa Rica: see [`idenumbra_cri::components`].
    #[cfg(feature = "cri")]
    CRI(idenumbra_cri::Components),
    /// Ecuador: see [`idenumbra_ecu::components`].
    #[cfg(feature = "ecu")]
    ECU(idenumbra_ecu::Components),
    /// Guatemala: see [`idenumbra_gtm::components`].
    #[cfg(feature = "gtm")]
    GTM(idenumbra_gtm::Components),
    /// Honduras: see [`idenumbra_hnd::components`].
    #[cfg(feature = "hnd")]
    HND(idenumbra_hnd::Components),
    /// Mexico: see [`idenumbra_mex::components`].
    #[cfg(feature = "mex")]
    MEX(idenumbra_mex::Components),
    /// Nicaragua: see [`idenumbra_nic::components`].
    #[cfg(feature = "nic")]
    NIC(idenumbra_nic::Components),
    /// Panama: see [`idenumbra_pan::components`].
    #[cfg(feature = "pan")]
    PAN(idenumbra_pan::Components),
    /// Peru: see [`idenumbra_per::components`].
    #[cfg(feature = "per")]
    PER(idenumbra_per::Components),
    /// Uruguay: see [`idenumbra_ury::components`].
    #[cfg(feature = "ury")]
    URY(idenumbra_ury::Components),
    /// Venezuela: see [`idenumbra_ven::components`].
    #[cfg(feature = "ven")]
    VEN(idenumbra_ven::Components),
}

/// Result of a successful validation: the document kind that was validated and,
/// when the number encodes any, its country-specific [`CountryComponents`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    /// Whether the value passed validation. Always `true` here, since this
    /// struct is only built on success.
    pub valid: bool,
    /// Validated document kind when it is known (`"id_card"`, `"ruc"`,
    /// `"passport"`, ...). `None` when the country validator did not report it.
    pub kind: Option<&'static str>,
    /// Parts encoded in the number, if any.
    ///
    /// They are extracted by inferring the document kind from the value, so
    /// when a number is valid as two documents they may describe the inferred
    /// one rather than the `kind` passed to [`Idenumbra::validate`].
    pub components: Option<CountryComponents>,
}

static COMPONENTS: OnceLock<BTreeMap<&'static str, ComponentsFn>> = OnceLock::new();

fn components_registry() -> &'static BTreeMap<&'static str, ComponentsFn> {
    COMPONENTS.get_or_init(|| {
        #[allow(unused_mut)]
        let mut m: BTreeMap<&'static str, ComponentsFn> = BTreeMap::new();
        #[cfg(feature = "arg")]
        m.insert("ARG", |value| {
            idenumbra_arg::components(value).map(|c| c.map(CountryComponents::ARG))
        });
        #[cfg(feature = "bol")]
        m.insert("BOL", |value| {
            idenumbra_bol::components(value).map(|c| c.map(CountryComponents::BOL))
        });
        #[cfg(feature = "bra")]
        m.insert("BRA", |value| {
            idenumbra_bra::components(value).map(|c| c.map(CountryComponents::BRA))
        });
        #[cfg(feature = "cri")]
        m.insert("CRI", |value| {
            idenumbra_cri::components(value).map(|c| c.map(CountryComponents::CRI))
        });
        #[cfg(feature = "ecu")]
        m.insert("ECU", |value| {
            idenumbra_ecu::components(value).map(|c| c.map(CountryComponents::ECU))
        });
        #[cfg(feature = "gtm")]
        m.insert("GTM", |value| {
            idenumbra_gtm::components(value).map(|c| c.map(CountryComponents::GTM))
        });
        #[cfg(feature = "hnd")]
        m.insert("HND", |value| {
            idenumbra_hnd::components(value).map(|c| c.map(CountryComponents::HND))
        });
        #[cfg(feature = "mex")]
        m.insert("MEX", |value| {
            idenumbra_mex::components(value).map(|c| c.map(CountryComponents::MEX))
        });
        #[cfg(feature = "nic")]
        m.insert("NIC", |value| {
            idenumbra_nic::components(value).map(|c| c.map(CountryComponents::NIC))
        });
        #[cfg(feature = "pan")]
        m.insert("PAN", |value| {
            idenumbra_pan::components(value).map(|c| c.map(CountryComponents::PAN))
        });
        #[cfg(feature = "per")]
        m.insert("PER", |value| {
            idenumbra_per::components(value).map(|c| c.map(CountryComponents::PER))
        });
        #[cfg(feature = "ury")]
        m.insert("URY", |value| {
            idenumbra_ury::components(value).map(|c| c.map(CountryComponents::URY))
        });
        #[cfg(feature = "ven")]
        m.insert("VEN", |value| {
            idenumbra_ven::components(value).map(|c| c.map(CountryComponents::VEN))
        });
        m
    })
}

static REGISTRY: OnceLock<BTreeMap<&'static str, (InferFn, TypedFn)>> = OnceLock::new();

fn registry() -> &'static BTreeMap<&'static str, (InferFn, TypedFn)> {
    REGISTRY.get_or_init(|| {
        #[allow(unused_mut)]
        let mut m: BTreeMap<&'static str, (InferFn, TypedFn)> = BTreeMap::new();
        #[cfg(feature = "arg")]
        {
            m.insert(
                "ARG",
                (
                    |value| idenumbra_arg::validate(value).map(|_| ()),
                    |value, kind| idenumbra_arg::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "bol")]
        {
            m.insert(
                "BOL",
                (
                    |value| idenumbra_bol::validate(value).map(|_| ()),
                    |value, kind| idenumbra_bol::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "bra")]
        {
            m.insert(
                "BRA",
                (
                    |value| idenumbra_bra::validate(value).map(|_| ()),
                    |value, kind| idenumbra_bra::validate_with(value, kind).map(|_| ()),
                ),
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
                (
                    |value| idenumbra_cri::validate(value).map(|_| ()),
                    |value, kind| idenumbra_cri::validate_with(value, kind).map(|_| ()),
                ),
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
                (
                    |value| idenumbra_ecu::validate(value).map(|_| ()),
                    |value, kind| idenumbra_ecu::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "gtm")]
        {
            m.insert(
                "GTM",
                (
                    |value| idenumbra_gtm::validate(value).map(|_| ()),
                    |value, kind| idenumbra_gtm::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "hnd")]
        {
            m.insert(
                "HND",
                (
                    |value| idenumbra_hnd::validate(value).map(|_| ()),
                    |value, kind| idenumbra_hnd::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "mex")]
        {
            m.insert(
                "MEX",
                (
                    |value| idenumbra_mex::validate(value).map(|_| ()),
                    |value, kind| idenumbra_mex::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "nic")]
        {
            m.insert(
                "NIC",
                (
                    |value| idenumbra_nic::validate(value).map(|_| ()),
                    |value, kind| idenumbra_nic::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "pan")]
        {
            m.insert(
                "PAN",
                (
                    |value| idenumbra_pan::validate(value).map(|_| ()),
                    |value, kind| idenumbra_pan::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "per")]
        {
            m.insert(
                "PER",
                (
                    |value| idenumbra_per::validate(value).map(|_| ()),
                    |value, kind| idenumbra_per::validate_with(value, kind).map(|_| ()),
                ),
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
                (
                    |value| idenumbra_ury::validate(value).map(|_| ()),
                    |value, kind| idenumbra_ury::validate_with(value, kind).map(|_| ()),
                ),
            );
        }
        #[cfg(feature = "ven")]
        {
            m.insert(
                "VEN",
                (
                    |value| idenumbra_ven::validate(value).map(|_| ()),
                    |value, kind| idenumbra_ven::validate_with(value, kind).map(|_| ()),
                ),
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
    pub fn validate(country: &str, value: &str, kind: &str) -> Result<Validation, ValidationError> {
        lookup(country)?.1(value, kind)?;
        let components = Idenumbra::components(country, value)?;
        // Aliases collapse to the name the country crates report.
        let kind = match kind {
            "id_card" | "identity_card" | "citizen_id" | "cedula" => Some("id_card"),
            "ruc" => Some("ruc"),
            "passport" => Some("passport"),
            "dni" => Some("dni"),
            "cuit" => Some("cuit"),
            "cuil" => Some("cuil"),
            "cpf" => Some("cpf"),
            "cnpj" => Some("cnpj"),
            "rut" | "run" => Some("rut"),
            "curp" => Some("curp"),
            "rfc" => Some("rfc"),
            "cui" | "dpi" => Some("cui"),
            "nit" => Some("nit"),
            "dui" => Some("dui"),
            "ci" => Some("ci"),
            "ce" => Some("ce"),
            "dimex" => Some("dimex"),
            "nite" => Some("nite"),
            "cedula_juridica" => Some("cedula_juridica"),
            "rif" => Some("rif"),
            _ => None,
        };
        Ok(Validation {
            valid: true,
            kind,
            components,
        })
    }

    /// Validate `value` letting the country validator infer the document kind.
    pub fn infer(country: &str, value: &str) -> Result<Validation, ValidationError> {
        lookup(country)?.0(value)?;
        let components = Idenumbra::components(country, value)?;
        let kind = match components {
            #[cfg(feature = "ecu")]
            Some(CountryComponents::ECU(idenumbra_ecu::Components::IdCard(_))) => Some("id_card"),
            #[cfg(feature = "ecu")]
            Some(CountryComponents::ECU(idenumbra_ecu::Components::Ruc(_))) => Some("ruc"),
            None => {
                #[cfg(feature = "ecu")]
                if country.trim().eq_ignore_ascii_case("ECU") {
                    Some("passport")
                } else {
                    None
                }
                #[cfg(not(feature = "ecu"))]
                None
            }
            _ => None, // other countries' components do not imply a kind here
        };
        Ok(Validation {
            valid: true,
            kind,
            components,
        })
    }

    /// Components encoded in a validated `value`.
    ///
    /// `Ok(None)` when the input is valid but has no structured components (a
    /// passport) or when the country's validator does not expose them.
    pub fn components(
        country: &str,
        value: &str,
    ) -> Result<Option<CountryComponents>, ValidationError> {
        lookup(country)?;
        let code = country.trim().to_ascii_uppercase();
        match components_registry().get(code.as_str()) {
            Some(parse) => parse(value),
            None => Ok(None),
        }
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

    #[test]
    fn typed_kind_is_reported() {
        // The kind the caller passed is echoed back as the canonical name.
        #[cfg(feature = "arg")]
        {
            assert_eq!(
                Idenumbra::validate("ARG", "12345678", "dni").map(|v| v.kind),
                Ok(Some("dni"))
            );
            assert_eq!(
                Idenumbra::validate("ARG", "20-12345678-6", "cuil").map(|v| v.kind),
                Ok(Some("cuil"))
            );
        }
        #[cfg(feature = "ecu")]
        {
            assert_eq!(
                Idenumbra::validate("ECU", "1712345675", "identity_card").map(|v| v.kind),
                Ok(Some("id_card"))
            );
        }
    }

    #[cfg(feature = "ecu")]
    #[test]
    fn components_ecu() {
        match Idenumbra::components("ecu", "1712345675") {
            Ok(Some(CountryComponents::ECU(idenumbra_ecu::Components::IdCard(card)))) => {
                assert_eq!(card.province.code, 17);
                assert_eq!(card.check_digit, 5);
            }
            other => panic!("expected a cédula, got {other:?}"),
        }
        assert_eq!(Idenumbra::components("ECU", "A123456"), Ok(None));
        assert!(Idenumbra::components("ECU", "1712345676").is_err());
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
