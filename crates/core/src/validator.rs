//! Shared validator contract.

/// Contract implemented by every country validator crate.
///
/// All validators expose the same surface: a country code and a `validate`
/// function taking the raw identification string. This allows the orchestrator
/// (and any caller) to invoke validators interchangeably.
pub trait Validator {
    /// ISO 3166-1 alpha-3 code handled by this validator.
    const COUNTRY: &'static str;

    /// Result type returned by a successful validation. Countries that do not
    /// expose extra information use `()`.
    type Validation;

    /// Validate `value`. Returns `Ok` with the validation result on success.
    fn validate(value: &str) -> Result<Self::Validation, crate::ValidationError>;

    /// Validate `value` as an explicit document `kind` (e.g. `"id_card"`,
    /// `"ruc"`, `"passport"`), skipping type inference. Use [`Self::validate`]
    /// when the kind is unknown. The default implementation ignores `kind` and
    /// delegates to [`Self::validate`].
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, crate::ValidationError> {
        let _ = kind;
        Self::validate(value)
    }
}

/// Generate the zero-sized marker struct and [`Validator`] implementation for a
/// country crate, removing the per-crate boilerplate.
///
/// `$marker` is the marker type name (e.g. `IdenumbraARG`), `$code` the ISO
/// 3166-1 alpha-3 code. The generated `validate`/`validate_with` delegate to
/// the crate's free functions of the same names, which must be in scope.
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! country_validator {
    ($marker:ident, $code:literal) => {
        /// Zero-sized marker type implementing [`$crate::Validator`].
        pub struct $marker;

        impl $crate::Validator for $marker {
            const COUNTRY: &'static str = $code;
            type Validation = ();
            fn validate(value: &str) -> Result<(), $crate::ValidationError> {
                validate(value)
            }
            fn validate_with(value: &str, kind: &str) -> Result<(), $crate::ValidationError> {
                crate::validate_with(value, kind)
            }
        }
    };
}
