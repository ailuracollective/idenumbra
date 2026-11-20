# API reference

This document summarizes the public API of `idenumbra-core` and the `idenumbra`
orchestrator. For the full, generated reference run:

```bash
cargo doc --workspace --all-features --no-deps
```

The rendered docs are also published on docs.rs:
[`idenumbra`](https://docs.rs/idenumbra),
[`idenumbra-core`](https://docs.rs/idenumbra-core).

## `idenumbra-core`

Dependency-free, `no_std`. Shared contract used by every country crate.

### Modules

- `country` — ISO 3166-1 alpha-3 constants (`ARG`, `BOL`, …, `VEN`) and
  `canonical(code) -> Option<&'static str>`.
- `error` — `ValidationError`.
- `validator` — `Validator` trait and `country_validator!` macro.

### `Validator` trait

```rust
pub trait Validator {
    const COUNTRY: &'static str;
    type Validation;
    fn validate(value: &str) -> Result<Self::Validation, ValidationError>;
    fn validate_with(value: &str, kind: &str) -> Result<Self::Validation, ValidationError>;
    fn document_kinds() -> &'static [&'static str];
}
```

- `validate` infers the document kind from the input.
- `validate_with` validates as an explicit `kind` (`"id_card"`, `"ruc"`,
  `"passport"`, `"dni"`, `"cuit"`, …).
- The default `validate_with` ignores `kind` and delegates to `validate`;
  kind-aware crates override it.

### `ValidationError`

`Empty`, `InvalidLength { expected, found }`, `NonNumeric { index }`,
`NonAlphanumeric { index }`, `UnknownCountry`,
`ValidatorNotAvailable { country }`, `CheckDigitMismatch { expected, received }`,
`UnrepresentableCheckDigit`, `InvalidFormat`, `InvalidRegion { region }`.

Implements `core::fmt::Display` and `core::error::Error`. Derives
`Debug, Clone, Copy, PartialEq, Eq, Hash`.

### Country codes

Constants are `&'static str`. `canonical(code)` trims and compares
case-insensitively, returning the canonical constant or `None`. It does not
check whether the validator is compiled in.

### `country_validator!` macro

```rust
idenumbra_core::country_validator!(IdenumbraECU, "ECU");
```

Generates the zero-sized marker struct and its `Validator` impl.

## Country crates (`idenumbra-<code>`)

Each crate is `no_std` and exposes:

```rust
pub fn validate(value: &str) -> Result<Validation, ValidationError>;
pub fn validate_with(value: &str, kind: &str) -> Result<Validation, ValidationError>;
pub fn components(value: &str) -> Result<Option<Components>, ValidationError>;
pub struct IdenumbraXXX; // marker implementing Validator
```

Use them directly for a smaller dependency graph, or through the orchestrator.

## `idenumbra` orchestrator

```rust
pub struct Idenumbra;
```

### Discovery

- `supported_countries() -> &'static [&'static str]`
- `is_available(country) -> bool`

### Validation

- `infer(country, value) -> Result<Validation, ValidationError>`
- `validate(country, value, kind) -> Result<Validation, ValidationError>`
- `components(country, value) -> Result<Option<CountryComponents>, ValidationError>`
- `classify(country, value) -> Result<Vec<&'static str>, ValidationError>`

### Candidates

- `candidates(value) -> Vec<&'static str>`
- `candidates_with(value, kind) -> Vec<&'static str>`

### Batch (lazy iterators)

- `validate_batch(country, inputs)`
- `validate_batch_with(country, inputs, kind)`
- `validate_batch_typed(country, inputs)` — items `(value, kind)`
- `validate_batch_mixed(country, inputs)` — items `(value, Option<kind>)`

Empty input yields zero results. Unknown or disabled countries surface as
`Err` per item.

### `Validation`

```rust
pub struct Validation {
    pub valid: bool,                       // always true on Ok
    pub kind: Option<&'static str>,
    pub components: Option<CountryComponents>,
}
```

### `CountryComponents`

Feature-gated enum wrapping each country's component type. ARG, BOL, BRA, CRI,
ECU, GTM, HND, MEX, NIC, PAN, PER, URY and VEN register a parser.

### Re-exports

`ValidationError`, `Validator`, and (when enabled) each marker type
`IdenumbraARG`, …, `IdenumbraVEN`.

### Feature flags

`arg`, `bol`, `bra`, `chl`, `col`, `cri`, `cub`, `dom`, `ecu`, `gtm`, `hnd`,
`mex`, `nic`, `pan`, `per`, `pry`, `slv`, `ury`, `ven`, `all`. Defaults to
empty.

### Notes

- Country codes are trimmed and uppercased by the orchestrator; the value is
  not trimmed (country validators trim it themselves).
- The registry is built lazily on first use (`OnceLock`).
