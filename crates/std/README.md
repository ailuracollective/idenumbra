# idenumbra

Orchestrator for idenumbra country validators.

## Documents

Feature flags: `arg`, `bol`, `bra`, `chl`, `col`, `cri`, `cub`, `dom`, `ecu`, `gtm`, `hnd`, `mex`, `nic`, `pan`, `per`, `pry`, `slv`, `ury`, `ven`, `all`.
Registry built lazily (OnceLock). `supported_countries()`, `is_available()`, `infer(country, value)`, `validate(country, value, kind)` and `components(country, value)` (country-specific parts for the validators that expose them).

## References

- Cargo features pattern.
