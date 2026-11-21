# idenumbra

Orchestrator for idenumbra country validators. Feature-gated registry with lazy
initialization, untyped and typed entry points, and a shared error type.

## Contents

- [Feature flags](#feature-flags)
- [API](#api)
- [Registry](#registry)
- [Usage](#usage)

## Feature flags

`arg`, `bol`, `bra`, `chl`, `col`, `cri`, `cub`, `dom`, `ecu`, `gtm`, `hnd`,
`mex`, `nic`, `pan`, `per`, `pry`, `slv`, `ury`, `ven`, `all`. Defaults to
empty: a country is compiled in only when its feature is enabled.

## API

```rust
pub struct Idenumbra;
```

### Discovery

- `Idenumbra::supported_countries() -> &'static [&'static str]` — country codes
  compiled into this build (sorted, cached).
- `Idenumbra::is_available(country) -> bool` — whether a validator for `country`
  is compiled in.

### Validation

- `Idenumbra::infer(country, value) -> Result<Validation, ValidationError>` —
  validate letting the country validator infer the document kind.
- `Idenumbra::validate(country, value, kind) -> Result<Validation, ValidationError>` —
  validate as an explicit document `kind` (e.g. `"id_card"`, `"ruc"`,
  `"passport"`, `"dni"`, `"cuit"`), skipping inference.
- `Idenumbra::components(country, value) -> Result<Option<CountryComponents>, ValidationError>` —
  country-specific parts encoded in a valid number.
- `Idenumbra::classify(country, value) -> Result<Vec<&'static str>, ValidationError>` —
  document kinds that validate `value` in `country`.

### Candidates

- `Idenumbra::candidates(value) -> Vec<&'static str>` — enabled country codes
  whose inferred validation rules accept `value` (candidates only).
- `Idenumbra::candidates_with(value, kind) -> Vec<&'static str>` — same, with an
  explicit document `kind`.

### Batch (lazy iterators)

- `Idenumbra::validate_batch(country, inputs)` — infers each kind.
- `Idenumbra::validate_batch_with(country, inputs, kind)` — shared `kind`.
- `Idenumbra::validate_batch_typed(country, inputs)` — items are `(value, kind)`.
- `Idenumbra::validate_batch_mixed(country, inputs)` — items are
  `(value, Option<kind>)`; `None` infers.

Each yields `(input_index, result)` in input order. Empty input yields zero
results.

### `Validation`

```rust
pub struct Validation {
    pub valid: bool,                       // always true on Ok
    pub kind: Option<&'static str>,        // reported kind when known
    pub components: Option<CountryComponents>,
}
```

### `CountryComponents`

Wraps the per-country component type. ARG, BOL, BRA, CRI, ECU, GTM, HND, MEX,
NIC, PAN, PER, URY and VEN register a parser, so `components` returns their
parts; it is `None` for the other countries and for documents that encode no
parts (passports, plain cédulas).

### Re-exports

`ValidationError`, `Validator`, and (when the feature is enabled) each marker
type `IdenumbraARG`, `IdenumbraBOL`, …, `IdenumbraVEN`.

## Registry

The registry is a `BTreeMap<&'static str, (InferFn, TypedFn)>` built lazily on
first use via `OnceLock`. `supported_countries()` is also cached. Disabled
countries are absent from both: `infer`/`validate` return
`ValidationError::ValidatorNotAvailable { country }` for a known-but-disabled
code, and `ValidationError::UnknownCountry` for an unknown code.

Country codes are trimmed and uppercased before lookup; **the value is not
trimmed** by the orchestrator (country validators trim it themselves).

## Usage

```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["all"] }
```

```rust
use idenumbra::Idenumbra;

match Idenumbra::infer("ARG", "12345678") {
    Ok(v) => println!("valid: kind={:?}", v.kind),
    Err(e) => println!("invalid: {e}"),
}

// Explicit kind bypasses inference.
Idenumbra::validate("ECU", "1712345675001", "ruc")?;

// Disabled or unknown countries.
assert!(!Idenumbra::is_available("ZZZ"));
```

Example binary: `cargo run --example basic --all-features`.

## API documentation

Generated reference: `cargo doc -p idenumbra --all-features --no-deps`.
Human-readable summary: [`docs/API.md`](../../docs/API.md).

## Build / test

```bash
cargo build -p idenumbra --all-features
cargo test -p idenumbra --all-features
cargo test -p idenumbra --features "ecu arg"
```

## References

- Cargo features pattern.
- `idenumbra-core` for the shared contract and error type.
