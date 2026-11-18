# idenumbra-per

Peru identification validator.

## Documents

- DNI (8 digits).
- RUC (11 digits, modulo 11).
- Carné de extranjería / passport (6-12 alphanumerics).

## Validation algorithm

DNI: 8 digits, no check digit. RUC: 11 digits starting with `10`, `15`, `17`
or `20`; weights `[5,4,3,2,7,6,5,4,3,2]` over the first 10 digits; check digit
`(11 - sum mod 11) mod 10` (remainder 0 gives 1, remainder 1 gives 0).

### Editable constants

- `RUC_PREFIXES = [b"10", b"15", b"17", b"20"]` (`src/validators/ruc.rs`) —
  issuer prefixes: `10` natural person with DNI, `15` foreign natural person,
  `17` natural person without DNI (implementation-only), `20` legal person.

## References

- **Official** — SUNAT, legislation and RUC definition: https://www.sunat.gob.pe/legislacion/ruc/
- **Official** — SUNAT, inscription in the RUC (natural person `10` + DNI,
  foreigner `15` + random number, legal person `20`): https://www.gob.pe/284-inscripcion-en-el-ruc
- **Official** — RENIEC, DNI questions (8 digits): https://identidad.reniec.gob.pe/preguntas-frecuentes-dni
- **Implementation** — python-stdnum `pe.ruc` (the `17` prefix is only
  documented by implementations; SUNAT does not list the prefix table):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/pe/ruc.py
- **Implementation** — https://es.wikipedia.org/wiki/Documento_Nacional_de_Identidad_(Per%C3%BA)

SUNAT publishes no prefix table and no check-digit specification; the RUC
number is only defined as "11 digits, unique". Official links were opened on
2026-11-17; the implementation links are public cross-checks, not dated.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["per"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("PER", "12345678")?;
```

## Test numbers (synthetic, not real)

- DNI: `12345678`
- RUC: `20123456786`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SUNAT modulo 11 rules.

## Notes

- Feature flag: `per`.
- Country code (ISO 3166-1 alpha-3): `PER`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
