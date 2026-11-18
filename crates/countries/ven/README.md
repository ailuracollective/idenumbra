# idenumbra-ven

Venezuela identification validator.

## Documents

- CI (6-8 digits).
- RIF (letter `V`/`E`/`J`/`P`/`G`/`C` plus 8 digits and a check digit).
- Passport (6-12 alphanumerics).

## Validation algorithm

CI: 6-8 digits, no check digit. RIF: the letter contributes
`V=4, E=8, J=12, P=16, G=20, C=12`; the 8 body digits use weights
`[3,2,7,6,5,4,3,2]`; the check digit is `11 - (sum mod 11)` where 10 and 11 are
written as `0`. Hyphens and spaces are ignored; lowercase letters are accepted.

### Editable constants

- `RIF_PREFIXES` (`src/validators/validate.rs`) — letter → checksum base:
  `V` venezolano, `E` extranjero, `J` jurídico, `P` pasaporte, `G` gobierno,
  `C` comuna/consejo comunal. `C` is an alias of `J` (same base `12`), so a
  comuna keeps the check digit of the `J` RIF it replaced.

## References

- **Official** — SENIAT: http://www.seniat.gob.ve/
- **Implementation** — python-stdnum `ve.rif`:
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/ve/rif.py
- **Implementation** — the `C` = `J` alias, checked against the SENIAT
  verifier: https://github.com/joseayram/utils/issues/3
- **Implementation** — https://es.wikipedia.org/wiki/Registro_%C3%9Anico_de_Informaci%C3%B3n_Fiscal

SENIAT publishes no document with the RIF algorithm or the letter table; the
`C` base is the one value the community has had to correct (it is `12`, not
`24`).

Official links were opened on 2026-11-17 (seniat.gob.ve answers `403` to
automated clients; open it in a browser); the implementation links are public
cross-checks, not dated.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["ven"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("VEN", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `12345678`
- RIF: `V-12345678-1` (`V123456781`)

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SENIAT modulo 11 rules.

## Notes

- Feature flag: `ven`.
- Country code (ISO 3166-1 alpha-3): `VEN`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
