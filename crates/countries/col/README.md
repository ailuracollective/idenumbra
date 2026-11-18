# idenumbra-col

Colombia identification validator.

## Documents

- CC (cédula de ciudadanía, 6-10 digits).
- NIT (9-digit body plus check digit, modulo 11).
- Passport (alphanumeric).

## Validation algorithm

CC: 6-10 digits, no check digit. NIT: weights from the right
`[3,7,13,17,19,23,29,37,41]` (the leftmost body digit is multiplied by 41,
per DIAN Orden Administrativa 4/1989); check digit is the remainder when it is
0 or 1, otherwise `11 - remainder`. Dots, hyphens and spaces are ignored.

## References

- https://www.dian.gov.co/ — checked 2026-11-17
- https://actualicese.com/guia-consulta-digito-de-verificacion-nit — checked 2026-11-17
- https://es.wikipedia.org/wiki/N%C3%BAmero_de_Identificaci%C3%B3n_Tributaria_(Colombia) — checked 2026-11-17

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["col"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("COL", "12345678")?;
```

## Test numbers (synthetic, not real)

- CC: `12345678`
- NIT: `900.657.800-0` (`9006578000`), validated with `Some("nit")`
- NIT: `987.654.321-7`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the validator algorithm; DIAN prime weights.

## Notes

- Feature flag: `col`.
- Country code (ISO 3166-1 alpha-3): `COL`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Cédulas and NITs overlap: 10-digit numbers without a kind are validated as cédulas, so use `validate_with("nit")` to check a NIT check digit.
- The DIAN publishes weights for bodies of up to 9 digits; that is the supported range.
