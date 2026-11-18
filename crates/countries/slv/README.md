# idenumbra-slv

El Salvador identification validator.

## Documents

- DUI (9 digits, modulo 10 check digit).
- NIT (14 digits, modulo 11 check digit).
- Passport (6-12 alphanumerics).

An adult natural person's NIT is their DUI, so a 9-digit number is also
accepted by `validate_with(.., "nit")`.

## Validation algorithm

DUI: weights `[9,8,7,6,5,4,3,2]` over the first 8 digits; check digit
`(10 - sum mod 10) mod 10`; all-zero numbers are rejected (they are placeholders,
never issued). The input must carry all 9 digits: a DUI stored as an integer
loses its leading zero, which encodes the municipality.

NIT: 14 digits whose first digit is `0`, `1` (nationals) or `9` (foreigners).
The correlative (positions 11-13) selects the weight table:

| Correlative | Weights | Check digit |
|---|---|---|
| `001`-`100` (legacy) | `14..2` | `(sum mod 11) mod 10` |
| `101`-`999` (current) | `[2,7,6,5,4,3,2,7,6,5,4,3,2]` | `((11 - sum mod 11) mod 11) mod 10` |

Below the cut-off either table is accepted. DGII publishes neither algorithm, so
the cut-off is reverse engineered and the 014-100 band is unobserved in every
public sample found: no NIT there is known to use the current table, but none is
known to rule it out either, and rejecting a legitimately issued number costs
more than the extra 1-in-10 chance of accepting a mistyped digit. From `101` up
only the current table applies (see the evidence below).

### Editable constants

- `TAXPAYER_TYPES = b"019"` (`src/validators/nit.rs`) — first NIT digit.
- `LEGACY_CORRELATIVE_MAX = 100` (`src/validators/nit.rs`) — correlative
  cut-off between the two weight tables.

## References

- **Official** — Ministerio de Hacienda, DTE JSON schemas: the `nit` field is
  `^([0-9]{14}|[0-9]{9})$`, i.e. 14 digits or the 9-digit DUI:
  https://factura.gob.sv/ (read through the byte-identical mirror
  https://github.com/herson/dte-json-schemas) — checked 2026-11-17.
- **Official** — Ministerio de Hacienda / DGII, facturación electrónica
  material: the emitter NIT field is numeric with a length of 14 or 9 digits:
  https://www.taiia.gob.sv/wp-content/uploads/2023/03/FE-TAIIA-v2-28-03-2023-con-videos.pdf
  — checked 2026-11-17.
- **Official (blocked)** — Ministerio de Hacienda, *Inscripción, modificación o
  reposición de tarjeta de sociedades por acciones simplificadas (SAS)*: returns
  HTTP 403 to automated requests, so it was not read. Even if opened it would
  confirm validity of previously assigned physical NITs, not the checksum:
  https://www.mh.gob.sv/servicios/inscripcion-modificacion-o-reposicion-de-tarjeta-de-sociedades-por-acciones-simplificadas-sas-en-los-registros-de-nit-e-iva/
  — attempted 2026-11-17, blocked
- **Implementation** — python-stdnum `sv.nit` (first digit `0`/`1`/`9`, both
  weight tables and the `<= 100` cut-off; DGII publishes none of it):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/sv/nit.py
- **Implementation** — https://github.com/avalon-tech/idsv-js (DUI weights and
  the DUI-as-NIT rule).
- **Implementation** — https://github.com/batressc/Batressc.DocumentValidators
  (`NITValidator.cs`: the same two weight tables and the `<= 100` cut-off, but
  only one table per number — no dual acceptance in the unobserved band).
- **Evidence** — 395 distinct 14-digit numbers were taken from Salvadoran
  government supplier and procurement PDFs, collected in
  https://github.com/arthurdejong/python-stdnum/issues/133 and re-checked
  against both tables on 2026-11-17: one is not a NIT (first digit `6`), and of
  the remaining 394, 382 pass at least one table; of the 272 with a correlative
  of 101 or more, 271 match the current table and one (correlative 105) matches
  only the legacy table and is treated as a transcription error in the source
  PDF; all 109 with a correlative of 13 or less match the legacy table; the two
  samples in between (both correlative 014) discriminate nothing — one
  satisfies both tables and the other neither. No public NIT in 015-100 was
  found to settle the cut-off, so both tables are accepted up to 100.

DGII publishes neither the NIT nor the DUI check-digit algorithm, so both are
reverse-engineered from public implementations and sample numbers, not from a
specification. No official NIT generation or checksum specification was found
(searched 2026-11-17). The Ministry's 2026 SAS registration service would only
confirm that previously assigned physical NITs remain valid; it could not be
opened in this session (HTTP 403) and in any case would not be evidence for the
weights or the correlative cut-off.

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
- NIT, legacy correlative: `0614-010120-001-0`
- NIT cut-off boundary: `0614-010120-100-1` (legacy table) and
  `0614-010120-102-1` (current table)

These values are **synthetic** and do **not** correspond to any real person or
company, except `0614-050707-104-8`, which is the fictitious sample published in
the python-stdnum docs. The legacy correlative value was generated locally with
this crate's validation algorithm.

## Notes

- Feature flag: `slv`.
- Country code (ISO 3166-1 alpha-3): `SLV`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- A DUI is a valid NIT for natural persons, so the `nit` kind accepts both 9 and
  14 digits.
