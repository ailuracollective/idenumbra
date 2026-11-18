# idenumbra-chl

Chile identification validator.

## Documents

- RUT/RUN (1-8 digit body plus check digit `0-9` or `K`).

## Validation algorithm

RUT/RUN: modulo 11 with weights 2..7 cycling from the right; check digit
`11 - (sum mod 11)`, where `11 -> 0` and `10 -> K`. Dots, hyphens and spaces
are ignored; lowercase `k` is accepted.

## References

- https://www.sii.cl/preguntas_frecuentes/catastro/001_012_5548.htm — checked 2026-11-17
- https://es.wikipedia.org/wiki/Rol_%C3%9Anico_Tributario — checked 2026-11-17
- https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/cl/rut.py — checked 2026-11-17

The SII does not publish the check-digit algorithm; it is the long-standing public specification, cross-checked against python-stdnum.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["chl"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("CHL", "12345678")?;
```

## Test numbers (synthetic, not real)

- RUT/RUN: `12.345.678-5` (`123456785`)
- RUT/RUN with check digit K: `12.345.670-K` (`12345670K`)

These values are **synthetic** and do **not** correspond to any real person. Generated locally with this crate's validation algorithm. Source: generated locally with the validator algorithm; SII modulo 11 rules.

## Notes

- Feature flag: `chl`.
- Country code (ISO 3166-1 alpha-3): `CHL`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
