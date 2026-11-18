# idenumbra-core

Core contract shared by every idenumbra country validator.

## Documents

- `Validator` trait: `const COUNTRY` + `fn validate(value) -> Result<(), ValidationError>` + `fn validate_with(value, kind)`.
- `ValidationError`: common error enum (Empty, InvalidLength, NonNumeric, NonAlphanumeric, UnknownCountry, ValidatorNotAvailable, CheckDigitMismatch, etc.).
- Country code constants (ISO 3166-1 alpha-3) and `canonical()`.

## References

- ISO 3166-1 alpha-3 country codes.
