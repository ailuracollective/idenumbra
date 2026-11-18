# idenumbra-arg

Argentina identification validator.

## Documents

- DNI (7-8 digits).
- CUIT/CUIL (11 digits, modulo 11).

## Validation algorithm

DNI: 7-8 digits, no check digit (older numbers have 7); all-zero placeholders
are rejected. CUIT/CUIL: 11 digits, weights `[5,4,3,2,7,6,5,4,3,2]`, check
digit `11 - (sum mod 11)` where `11 -> 0` and `10 -> 9` (remainder 1 is issued
as 9, it is not rejected). The two-digit type prefix is not enforced: RG
Conjunta AFIP-ANSES 5007/2021 assigns `20`, `23`, `24`, `27` — "or those
determined in the future" — at random since 2022, and `25`/`26` are contingency
prefixes. Dots, hyphens, slashes and spaces are ignored.

Scale: the check digit catches single-digit typos and most transpositions; it
does not prove the number was issued. A valid CUIT does not mean the taxpayer
exists or is active.

## References

- **Official** — Resolución General Conjunta AFIP-ANSES 5007/2021 (BO
  10/06/2021), art. 1: the CUIT/CUIL prefix `20`, `23`, `24`, `27` "o los que en
  el futuro se determinen" is assigned at random, not by sex:
  https://www.argentina.gob.ar/normativa/nacional/norma-350854/texto
- **Official** — ARCA/AFIP, CUIT concepts: https://servicioscf.afip.gob.ar/publico/abc/ABCpaso2.aspx?id_nivel1=3036&id_nivel2=3040&p=Conceptos%20b
- **Official** — ARCA, constancia de inscripción: https://www.afip.gob.ar/constanciadeinscripcion/
- **Implementation** — python-stdnum `ar.cuit` (cross-check of the weights and
  the `10 -> 9` mapping):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/ar/cuit.py

No ARCA norm publishes the check-digit algorithm or the full prefix table
(`30`/`33`/`34` for legal persons, `50`/`51`/`55` for foreign ones); that is why
the prefix is not enforced and the algorithm is cross-checked against public
implementations.

Official links were opened on 2026-11-17; the implementation link is a public
cross-check, not dated.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["arg"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("ARG", "12345678")?;
```

## Test numbers (synthetic, not real)

- DNI: `12345678`
- CUIT/CUIL: `20-12345678-6` (`20123456786`)
- CUIT/CUIL with check digit 9 (remainder 1): `20-00000001-9`
- CUIT/CUIL with a contingency prefix: `25-12345678-8`

These values are **synthetic** and do **not** correspond to any real person. Generated locally with this crate's validation algorithm. Source: generated locally with the validator algorithm; CUIT prefixes from AFIP.

## Notes

- Feature flag: `arg`.
- Country code (ISO 3166-1 alpha-3): `ARG`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- `validate_with("dni" | "cuit" | "cuil")` skips inference; CUIT/CUIL accept the usual formatting.
- The DNI carries no check digit, so only its length (7-8 digits) is checked.
