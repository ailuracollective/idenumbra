# idenumbra-core

Core contract shared by every idenumbra country validator. Dependency-free and
`no_std`-compatible (Rust 1.81+).

## Modules

- `country` — ISO 3166-1 alpha-3 country code constants (`ARG`, `BOL`, …,
  `VEN`) and `canonical(code) -> Option<&'static str>`.
- `error` — the shared `ValidationError` enum.
- `validator` — the `Validator` trait and the `country_validator!` macro.

## `Validator` trait

```rust
pub trait Validator {
    const COUNTRY: &'static str;
    type Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError>;
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError> { ... }
}
```

- `COUNTRY` is the ISO 3166-1 alpha-3 code handled by the validator.
- `Validation` is `()` for countries that expose no extra information.
- `validate` infers the document kind from the input.
- `validate_with` validates as an explicit document `kind` (`"id_card"`, `"ruc"`,
  `"passport"`, `"dni"`, `"cuit"`, …), skipping inference. The default
  implementation ignores `kind` and delegates to `validate`; kind-aware crates
  override it.

## `ValidationError`

| Variant | Meaning |
|---|---|
| `Empty` | Input is empty after trimming. |
| `InvalidLength { expected, found }` | Length does not match an accepted value. |
| `NonNumeric { index }` | A non-ASCII-digit character at `index`. |
| `NonAlphanumeric { index }` | A non-ASCII-alphanumeric character at `index`. |
| `UnknownCountry` | The country code is not a known ISO code. |
| `ValidatorNotAvailable { country }` | The country is known but its validator is not compiled in. |
| `CheckDigitMismatch { expected, received }` | Check digit does not match the computed one. |
| `UnrepresentableCheckDigit` | The body yields a check digit that cannot be represented. |
| `InvalidFormat` | The input does not match the expected document format. |
| `InvalidRegion { region }` | An invalid province/region code. |

`ValidationError` implements `core::fmt::Display` and `core::error::Error`.

## Country codes

Constants are `&'static str` (e.g. `idenumbra_core::ARG`). `canonical(code)`
trims and compares case-insensitively, returning the canonical constant when
the code is a known country, otherwise `None`. It does **not** check whether the
validator is compiled in — that is the orchestrator's job.

## `country_validator!` macro

Generates the zero-sized marker struct and its `Validator` impl, delegating to
the crate's free `validate` / `validate_with` functions:

```rust
idenumbra_core::country_validator!(IdenumbraECU, "ECU");
```

This removes the per-crate boilerplate for the marker type.

## Usage

As a dependency of a country crate (already wired by every `idenumbra-<code>`
crate):

```toml
[dependencies]
idenumbra-core = "0.1.0-alpha.1"
```

Implementing a validator by hand:

```rust
#![no_std]
use idenumbra_core::{ValidationError, Validator};

pub fn validate(value: &str) -> Result<(), ValidationError> {
    let v = value.trim();
    if v.is_empty() { return Err(ValidationError::Empty); }
    // ... country-specific checks ...
    Ok(())
}

pub fn validate_with(value: &str, kind: &str) -> Result<(), ValidationError> {
    match kind {
        "id_card" => validate(value),
        _ => Err(ValidationError::InvalidFormat),
    }
}

idenumbra_core::country_validator!(IdenumbraXXX, "XXX");
```

## `no_std` notes

- No dependencies, no `alloc`, no `std`.
- `ValidationError` derives `Debug, Clone, Copy, PartialEq, Eq, Hash`.
- Errors implement `core::error::Error`, so they work with `?` in `no_std`
  contexts that provide an error type.

## References

- ISO 3166-1 alpha-3 country codes.
- Rust `no_std` / MSRV 1.81 (workspace).
