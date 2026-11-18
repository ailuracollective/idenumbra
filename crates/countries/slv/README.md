# idenumbra-slv

El Salvador identification validator.

## Documents

- DUI (9 digits, modulo 10).
- NIT (14 digits, modulo 11).
- Passport (6-12 alphanumerics).

## Validation algorithm

DUI: weights `[9,8,7,6,5,4,3,2]` over the first 8 digits; check digit
`(10 - sum mod 10) mod 10`; all-zero numbers are rejected. NIT: 14 digits whose
first digit is `0`, `1` (nationals) or `9` (foreigners); the correlative
(positions 11-13) selects the DGII weight table: old numbers (`<= 100`) use
weights `14..2` and `(sum mod 11) mod 10`, new numbers use
`[2,7,6,5,4,3,2,7,6,5,4,3,2]` and `((11 - sum mod 11) mod 11) mod 10`.

### Editable constants

- `TAXPAYER_TYPES = b"019"` (`src/validators/nit.rs`) — first NIT digit.
- `LEGACY_CORRELATIVE_MAX = b"100"` (`src/validators/nit.rs`) — correlative
  cut-off between the two weight tables.

## References

- **Official** — Ministerio de Hacienda, NIT service (the DUI is the NIT of
  adult natural persons since 2022): https://www.mh.gob.sv/servicios/inscripcion-modificacion-o-reposicion-de-nit-persona-natural/
- **Official** — Registro Nacional de las Personas Naturales, DUI: https://www.rnpn.gob.sv/
- **Implementation** — python-stdnum `sv.nit` (first digit `0`/`1`/`9` and the
  two weight tables selected by the correlative; DGII does not publish either):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/sv/nit.py
- **Implementation** — https://github.com/avalon-tech/idsv-js

DGII publishes neither the NIT nor the DUI check-digit algorithm; the two
weight tables and the `<= 100` correlative cut-off are reverse engineered, so
treat them as implementation notes rather than specification.

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
idenumbra = { version = "0.1", features = ["slv"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("SLV", "12345678")?;
```

## Test numbers (synthetic, not real)

- DUI: `12345678-4` (`123456784`)
- NIT: `0614-050707-104-8`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the DGII modulo 11 rules.

## Notes

- Feature flag: `slv`.
- Country code (ISO 3166-1 alpha-3): `SLV`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Since 2021 a DUI is also a valid NIT for natural persons; the `nit` kind
  accepts a 9-digit DUI as well as the 14-digit NIT.
