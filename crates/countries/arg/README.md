# idenumbra-arg

Argentina identification validator.

## Documents

- DNI (7-8 digits).
- CUIT/CUIL (11 digits, modulo 11).

## Validation algorithm

DNI: 7-8 digits, no check digit. CUIT/CUIL: 11 digits, weights
`[5,4,3,2,7,6,5,4,3,2]`, check digit `11 - (sum mod 11)` where `11 -> 0` and
`10 -> 9` (remainder 1 is issued as 9, it is not rejected). The two-digit type
prefix is not enforced: since 2026 ARCA assigns it randomly to new numbers.
Dots, hyphens, slashes and spaces are ignored.

## References

- **Official** — Resolución General Conjunta AFIP-ANSES 5007/2021 (prefixes
  `20`, `23`, `24`, `27` and any future one are assigned at random):
  https://www.argentina.gob.ar/normativa/nacional/norma-350854/texto
- **Official** — ARCA/AFIP, CUIT concepts: https://servicioscf.afip.gob.ar/publico/abc/ABCpaso2.aspx?id_nivel1=3036&id_nivel2=3040&p=Conceptos%20b
- **Official** — ARCA, constancia de inscripción: https://www.afip.gob.ar/constanciadeinscripcion/
- **Implementation** — python-stdnum `ar.cuit`:
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/ar/cuit.py
- **Implementation** — https://es.wikipedia.org/wiki/Clave_%C3%9Anica_de_Identificaci%C3%B3n_Tributaria

No ARCA norm publishes the full prefix table (`30`/`33`/`34` for legal
persons, `50`/`51`/`55` for foreign ones); that is why the prefix is not
enforced. The check-digit algorithm is not published either.

Official links were opened on 2026-11-17; the implementation links are public
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
