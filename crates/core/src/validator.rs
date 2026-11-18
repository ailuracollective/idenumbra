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
