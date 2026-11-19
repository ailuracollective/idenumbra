# idenumbra

Workspace of Latin American identification validators. One `no_std` crate per
country, plus a shared core contract and an orchestrator with feature-gated
registration.

> Country codes and crate/feature names use the **ISO 3166-1 alpha-3** standard
> (e.g. `ARG`, `MEX`). Codes are matched case-insensitively.

## Crates

| Crate | Package | Description |
|---|---|---|
| `idenumbra-core` | `idenumbra-core` | Shared contract: `Validator` trait, `ValidationError`, country code constants, `canonical()`. `no_std`, dependency-free. |
| `idenumbra-<code>` (×19) | `idenumbra-arg`, … | One crate per country. `no_std`, implements `Validator`. |
| `idenumbra` | `idenumbra` | Orchestrator: feature-gated registry, lazy `BTreeMap`, untyped/typed entry points. |

## Quick start

Orchestrator (enable only the countries you need):

```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["arg", "mex"] }
```

```rust
use idenumbra::Idenumbra;

Idenumbra::infer("ARG", "12345678")?;                 // infers the document kind
Idenumbra::validate("ARG", "20-12345678-6", "cuit")?; // explicit kind

let v = Idenumbra::infer("MEX", "GALA850315HNERXN05")?;
println!("{:?}", v.kind); // Some("id_card") when reported, else None
```

Single country (no orchestrator, smaller dependency graph):

```toml
[dependencies]
idenumbra-arg = "0.1.0-alpha.1"
```

```rust
idenumbra_arg::validate("12345678")?;
idenumbra_arg::validate_with("20-12345678-6", "cuit")?;
```

## Supported countries

| Code | Country | Documents |
|---|---|---|
| ARG | Argentina | DNI, CUIT/CUIL |
| BOL | Bolivia | CI, NIT |
| BRA | Brazil | CPF, CNPJ |
| CHL | Chile | RUT/RUN |
| COL | Colombia | CC (cédula), NIT, passport |
| CRI | Costa Rica | Cédula física, cédula jurídica, NITE, DIMEX, passport |
| CUB | Cuba | Carné de identidad, passport |
| DOM | Dominican Republic | Cédula, passport |
| ECU | Ecuador | Cédula, RUC, passport |
| GTM | Guatemala | CUI/DPI, NIT, passport |
| HND | Honduras | DNI, RTN, passport |
| MEX | Mexico | CURP, RFC |
| NIC | Nicaragua | Cédula, passport |
| PAN | Panama | Cédula/RUC, RUC NT, passport |
| PER | Peru | DNI, RUC, carné/passport |
| PRY | Paraguay | CI, RUC, passport |
| SLV | El Salvador | DUI, NIT, passport |
| URY | Uruguay | CI, RUT, passport |
| VEN | Venezuela | CI, RIF, passport |

## Orchestrator API (`idenumbra`)

- `Idenumbra::supported_countries() -> &'static [&'static str]` — country codes compiled into this build.
- `Idenumbra::is_available(country) -> bool` — whether a validator for `country` is compiled in.
- `Idenumbra::infer(country, value) -> Result<Validation, ValidationError>` — validate letting the country validator infer the document kind.
- `Idenumbra::validate(country, value, kind) -> Result<Validation, ValidationError>` — validate as an explicit document `kind` (`"id_card"`, `"ruc"`, `"passport"`, `"dni"`, `"cuit"`, …).
- `Idenumbra::components(country, value) -> Result<Option<CountryComponents>, ValidationError>` — country-specific parts encoded in a valid number.
- `Idenumbra::validate_batch(country, inputs) -> impl Iterator<Item = (usize, Result<Validation, ValidationError>)>` — lazily validate a sequence of values, inferring each document kind; yields `(input_index, result)` in input order without collecting inputs or results.
- `Idenumbra::validate_batch_with(country, inputs, kind)` — same, with a single shared document `kind`.
- `Idenumbra::validate_batch_typed(country, inputs)` — each input is `(value, kind)`.
- `Idenumbra::validate_batch_mixed(country, inputs)` — each input is `(value, Option<kind>)`; `None` infers the kind.

Empty input yields zero results. Unknown or disabled countries surface as `Err` per item.

`Validation` carries `valid: bool` (always `true` on `Ok`), `kind: Option<&'static str>` and
`components: Option<CountryComponents>`. ARG, BOL, BRA, CRI, ECU, GTM, HND, MEX, NIC, PAN,
PER, URY and VEN expose components; `components` is `None` for the other countries and for
documents that encode no parts (passports, plain cédulas).

Country codes are trimmed and uppercased by the orchestrator; **the value is not trimmed**
(country validators trim it themselves).

## Feature flags

`arg`, `bol`, `bra`, `chl`, `col`, `cri`, `cub`, `dom`, `ecu`, `gtm`, `hnd`, `mex`,
`nic`, `pan`, `per`, `pry`, `slv`, `ury`, `ven`, `all`. Defaults to empty. Only enabled
validators are compiled in and registered; the registry is built lazily on first use
(`OnceLock`). A disabled country is not compiled and returns
`ValidationError::ValidatorNotAvailable`.

## Validation scope

`Ok` means the value is well formed and, where the issuing agency publishes one,
its check digit matches. It does **not** mean the number exists, is assigned, is
active, or belongs to the person presenting it: use the issuing registry (SII,
DIAN, SUNAT, DGI, SAR, …) for that.

Where the agency publishes no check digit, only the structure is validated:
BOL NIT, COL cédula, CRI cédula/DIMEX, CUB carné, HND RTN and PAN cédula/RUC (the
DGI check digits depend on the taxpayer type). For COL the NIT check digit and
for PRY the CI/RUC split require an explicit `kind`.

[`LIMITATIONS.md`](LIMITATIONS.md) indexes the residual risks per country and
links the official registry where a number can be verified.

## Editable constants

Values that come from a published catalogue (province codes, issuer prefixes,
reserved codes) live in named `const`s at the top of each validator file, so a
catalogue change is a one-line edit and never touches the algorithm. Each crate
README lists them with the source that backs the current value.

## Build / test / lint

```bash
make build        # cargo build --workspace
make build-all    # cargo build --workspace --all-features
make test         # cargo test --workspace
make test-all     # cargo test --workspace --all-features
make test-features FEATURES="arg"
make fmt          # cargo fmt --all
make clippy       # cargo clippy --workspace --all-features --examples
make doc          # cargo doc --workspace --all-features --no-deps
make example      # cargo run --example basic --all-features
```

Single country crate without the orchestrator:

```bash
cargo test -p idenumbra-arg
```

## References

Each crate's README lists the sources used for its validation algorithm,
official first and `implementation` cross-checks second. Country-specific
details (algorithms, constants, synthetic test values) live in
`crates/countries/<code>/README.md`, not here.
