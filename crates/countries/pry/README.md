# idenumbra-pry

Paraguay identification validator.

## Documents

- CI (6-8 digits).
- RUC (2-8 digit body plus check digit, modulo 11).

## Validation algorithm

CI: 6-8 digits, no check digit. RUC: modulo 11 with weights 2..11 cycling from
the right; the check digit is `11 - remainder`, where remainders 0 and 1 map to
`0` (DNIT/SET rule). Dots, hyphens and spaces are ignored.

## References

- https://www.dnit.gov.py/web/portal-institucional/documentos — checked 2026-11-17
- https://www.set.gov.py/ — checked 2026-11-17
- https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/py/ruc.py — checked 2026-11-17

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["pry"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("PRY", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `1234567`
- RUC: `1946520-3` (`19465203`)
- RUC: `80028061-0`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the DNIT/SET modulo 11 rules.

## Notes

- Feature flag: `pry`.
- Country code (ISO 3166-1 alpha-3): `PRY`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Without a kind, 6-7 digit numbers are treated as CI and 8-9 digit numbers as
  RUC; use `validate_with("ci" | "ruc")` to disambiguate.
