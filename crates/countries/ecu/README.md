# idenumbra-ecu

Ecuador identification validator.

## Documents

- Cédula (10 digits).
- RUC (13 digits).
- Passport (6-9 alphanumerics).

## Validation algorithm

Cédula: province `01-24` or `30` (birth registered abroad), third digit `0-5`,
modulo 10 with weights `[2,1,2,1,2,1,2,1,2]` (products above 9 have 9
subtracted). RUC: provinces `01-24`, `30` and `50` (provisional series issued
before 2010); third digit `0-5` is a natural person (valid cédula plus a
non-zero establishment), `9` is a private company (modulo 11 with weights
`[4,3,2,7,6,5,4,3,2]`), `6` is a public company (weights
`[3,2,7,6,5,4,3,2]`); `6` and `9` fall back to the other SRI variant. A
hyphen written right before the last digit is tolerated (`171234567-5`).

### Editable constants

- `MAX_PROVINCE_CODE = 24` (`src/validators/id_card.rs`) — provinces are
  `01`-`24`; edit when one is created or merged.
- `ABROAD_PROVINCE_CODE = 30` (`src/validators/id_card.rs`) — assigned to
  people whose birth was registered abroad (Registro Civil, Resolución 140,
  Registro Oficial 681 of 2012).
- `PROVISIONAL_CODE = 50` (`src/validators/ruc.rs`) — provisional series the
  Registro Civil issued before 2010; legacy RUCs still carry it.

## References

- **Official** — Registro Civil, *Informativo* (code `30` for births registered
  abroad, `50` for pre-2010 provisional numbers, Resolución 140 / RO 681):
  https://www.registrocivil.gob.ec/informativo/
- **Official** — INEC, División Política Administrativa (province codes `01`-`24`):
  https://www.ecuadorencifras.gob.ec/documentos/web-inec/Geografia_Estadistica/Micrositio_geoportal/index.html
- **Official** — SRI, RUC for natural persons: https://www.sri.gob.ec/ruc-personas-naturales
- **Official** — SRI, RUC for companies: https://www.sri.gob.ec/ruc-sociedades
- **Implementation** — python-stdnum `ec.ruc` (the SRI publishes neither the
  check-digit algorithm nor the third-digit table):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/ec/ruc.py
- **Implementation** — https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Ecuador)

The SRI does not publish the RUC check-digit algorithm or the third-digit
table; the cédula algorithm has no official publication either.

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
idenumbra = { version = "0.1", features = ["ecu"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("ECU", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `1712345675`
- RUC (natural person): `1712345675001`
- RUC (private company): `1790012344001`
- RUC (public company): `1760001200001`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SRI modulo 10/11 rules.

## Notes

- Feature flag: `ecu`.
- Country code (ISO 3166-1 alpha-3): `ECU`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
