# idenumbra

Workspace of Latin American identification validators.

> Country codes and crate/feature names use the **ISO 3166-1 alpha-3** standard
> (e.g. `ARG`, `MEX`). Codes are matched case-insensitively.

## Crates

- `idenumbra-core`: shared contract (`Validator` trait, `ValidationError`, country codes). `no_std`.
- Country validators: `idenumbra-arg`, `idenumbra-bol`, `idenumbra-bra`, `idenumbra-chl`, `idenumbra-col`, `idenumbra-cri`, `idenumbra-cub`, `idenumbra-dom`, `idenumbra-ecu`, `idenumbra-gtm`, `idenumbra-hnd`, `idenumbra-mex`, `idenumbra-nic`, `idenumbra-pan`, `idenumbra-per`, `idenumbra-pry`, `idenumbra-slv`, `idenumbra-ury`, `idenumbra-ven`.
- `idenumbra`: orchestrator with Cargo features.

## Usage

Generic (N validators):

```toml
[dependencies]
idenumbra = { version = "0.1", features = ["arg", "mex"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("ARG", "12345678")?;            // infers the document kind
Idenumbra::validate("ARG", "12345678", "dni")?;  // explicit kind
```

Single country (no orchestrator):

```toml
[dependencies]
idenumbra-ecu = "0.1"
```

```rust
idenumbra_ecu::validate("1712345675")?;
```

## Validation scope

`Ok` means the value is well formed and, where the issuing agency publishes one,
its check digit matches. It does **not** mean the number exists, is assigned, is
active, or belongs to the person presenting it: use the issuing registry (SII,
DIAN, SUNAT, DGI, SAR, …) for that.

Where the agency publishes no check digit, only the structure is validated:
BO NIT, CO cédula, CR cédula/DIMEX, CU carné, HN RTN and PA cédula/RUC (the
DGI check digits depend on the taxpayer type). For CO the NIT check digit and
for PY the CI/RUC split require an explicit `kind`.

## Feature flags

`arg`, `bol`, `bra`, `chl`, `col`, `cri`, `cub`, `dom`, `ecu`, `gtm`, `hnd`, `mex`, `nic`, `pan`, `per`, `pry`, `slv`, `ury`, `ven`, `all`. Only enabled validators are compiled in and registered. The registry is built lazily on first use.

## Editable constants

Values that come from a published catalogue (province codes, issuer prefixes,
reserved codes) live in named `const`s at the top of each validator file, so a
catalogue change is a one-line edit and never touches the algorithm. Each crate
README lists them with the source that backs the current value.

## References

Each crate's README lists the sources used for its validation algorithm,
official first and `implementation` cross-checks second.
