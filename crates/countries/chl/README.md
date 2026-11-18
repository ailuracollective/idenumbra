# idenumbra-chl

Chile identification validator.

## Documents

- RUT/RUN (1-8 digit body plus check digit `0-9` or `K`).

## Validation algorithm

RUT/RUN: modulo 11 with weights 2..7 cycling from the right; check digit
`11 - (sum mod 11)`, where `11 -> 0` and `10 -> K`. A 1-8 digit body plus one
check character, so 2-9 characters once compacted. Dots, hyphens and spaces are
ignored; lowercase `k` is accepted. All-zero bodies are rejected as placeholders.

## References

- https://www.sii.cl/declaraciones_juradas/ingreso_upload_f3500.html — checked 2026-11-17
  (official SII layout: "Rut Declarante debe cumplir Modulo 11", "Dígito
  verificador ... Módulo 11")
- https://www.sii.cl/preguntas_frecuentes/preguntas_generales/001_430_5071.htm — checked 2026-11-17
  (official SII FAQ on the "Dígito Verificador Erróneo" message; updated
  20/10/2025)
- https://www.sii.cl/factura_electronica/factura_mercado/formato_boleta_electronica.pdf — checked 2026-11-17
  (official "Formato Boletas Electrónicas", version 4.2 of 2025-09-08: the RUT
  field is a numeric body plus a check character 0-9 or `K`; it also names the
  generic `66.666.666-6`)
- https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/cl/rut.py — checked 2026-11-17
  (cross-check; the SII states that a "módulo 11" check exists but does not
  publish the weights, which are the long-standing public specification)

The SII prints the modulus and the `0-9`/`K` alphabet, but not the residue
mapping: `10 -> K` follows from modulo 11 because 10 is not a digit, and the
boleta format independently allows `K` as the check character.

`66.666.666-6`, the generic RUT the SII requires on boletas without a customer
RUT, passes this validator (its digit is correct); it identifies no person.

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
- RUT and RUN share the number and the check digit for natural persons; the
  same modulo 11 applies to both.
- A correct digit only means the number is well formed; existence is confirmed
  by the SII or the Registro Civil.
