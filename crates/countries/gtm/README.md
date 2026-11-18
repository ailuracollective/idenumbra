# idenumbra-gtm

Guatemala identification validator.

## Documents

- CUI/DPI (13 digits).
- NIT (2-12 digits, check character `0-9` or `K`).
- Passport (6-12 alphanumerics).

## Validation algorithm

CUI: layout `correlativo(8) + check digit + departamento(2) + municipio(2)`.
RENAP publishes the 13-digit layout and states that the last four digits are
the department and then the municipality of birth; it does not publish the
check-digit formula. As in the open-source implementations, the check digit is
`sum(first 8 digits × 2..9) mod 11`; a remainder of `10` is not a digit and is
rejected. The department must be `01-22` and the municipality must exist for
that department (INE codes run `01` to the last municipality of the department,
without gaps).

NIT: each body digit is multiplied by a factor starting at `body length + 1`
and decreasing; the check character is `(11 - total mod 11) mod 11`, where `11`
is written as `0` and `10` as `K`.

### Editable constants

- `MUNICIPALITIES_PER_DEPARTMENT` (`src/validators/cui.rs`) — municipality
  count per department, indexed from `01` (Guatemala); index `0` is unused, so
  department `00` is invalid. The table sums 340, the current INE total.
  Escuintla gained Sipacate (`14`, Decreto 4-2015, municipality 339) and
  Huehuetenango gained Petatán (`33`, Decreto 6-2015, municipality 340); keep
  the count in sync when another municipality is created.

## References

Official:

- RENAP, *¿Qué es el DPI y por qué tengo CUI?* — 13 digits: 8 assigned by
  RENAP, 1 check digit, 2 department, 2 municipality, in that order:
  https://www.renap.gob.gt/noticias/que-es-el-dpi-y-por-que-tengo-cui —
  checked 2026-11-17
- RENAP, *¿Qué es el CUI?*: https://www.renap.gob.gt/noticias/que-es-el-cui —
  checked 2026-11-17
- SAT, *Preguntas frecuentes — Información de Contribuyentes* (from
  2023-10-04 new NITs are the first 9 digits of the CUI):
  https://portal.sat.gob.gt/portal/preguntas-frecuentes/informacion-de-contribuyentes/ —
  checked 2026-11-17
- SAT, *¿Qué significa para el guatemalteco la armonización del CUI con el
  NIT?* (2025-03-31, Resolución SAT-DSI-393-2025):
  https://portal.sat.gob.gt/portal/noticias/que-significa-para-el-guatemalteco-la-armonizacion-del-cui-con-el-nit —
  checked 2026-11-17
- INE, *Estimaciones y proyecciones de población a nivel departamental y
  municipal* (22 departments, 340 municipalities):
  https://www.ine.gob.gt/ine/wp-content/uploads/2020/08/02-Presentacion-de-Resultados-Departamentales-y-Municipales.pdf —
  checked 2026-11-17
- Congreso de la República, Decreto 6-2015 (creates Petatán, Huehuetenango):
  https://www.congreso.gob.gt/assets/uploads/info_legislativo/decretos/2015/06-2015.pdf —
  checked 2026-11-17
- SEGEPLAN, *PDM-OT Sipacate, Escuintla* (Decreto 4-2015, municipality 339):
  https://portal.segeplan.gob.gt/segeplan/wp-content/uploads/2025/05/0514_PDM_OT_SIPACATE.pdf —
  checked 2026-11-17
- IGN, municipal boundaries layer metadata (codes `0514` and `1333` for
  Sipacate and Petatán): https://www.arcgis.com/home/item.html?id=6164b3415296419893534831bbe1c8a3 —
  checked 2026-11-17

Implementations (cross-checks, not authority):

- https://github.com/momotombodevs/stdnum-centroamerica (`gt.dpi`: same weights,
  same 340-municipality table, municipality geography note dated 2026-11-17)
- https://github.com/alextello/validador-dpi-nit (weights `2..9`, `mod 11`)
- https://github.com/aalonzolu/cui-gt (same formula, but 32 municipalities in
  Huehuetenango)
- Conflict: https://box4.dev/es/guatemala/validador-cui and
  https://administrativonexusvida.web.app/en/tools/procedures/guatemala-dpi-nit-validator
  describe the same rule as `11 - (sum mod 11)` with `10 -> 0`. Both forms agree
  except when the remainder is `10`; this crate rejects that case, like
  stdnum-centroamerica and alextello, because a NIT formed from those nine digits
  would end in `K` and a CUI cannot.

RENAP and SAT do not publish the CUI or NIT check-digit algorithms. The layout
is official (RENAP); the NIT length of 2 to 12 characters is reported from SAT's
FEL validation rules by
https://www.lookuptax.com/docs/tax-identification-number/guatemala-tax-id-guide
(a secondary source, checked 2026-11-17). The weights are the community
standard, documented here as such.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["gtm"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("GTM", "12345678")?;
```

## Test numbers (synthetic, not real)

- CUI/DPI: `1234567890101`
- CUI/DPI in Sipacate, Escuintla: `1234567890514`
- CUI/DPI in Petatán, Huehuetenango: `1234567891333`
- NIT: `12345679`
- NIT with check character K: `1023456K`
- NIT whose check character is 0: `12345660`

These values are **synthetic** and do **not** correspond to any real person or
company. They were generated locally with this crate's validation algorithm.

## Notes

- Feature flag: `gtm`.
- Country code (ISO 3166-1 alpha-3): `GTM`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- `validate_with("cui" | "dpi" | "nit" | "passport")` forces the document type.
- Since 2023-10-04 a natural person's NIT is the first 9 digits of the CUI, and
  SAT accepts the full 13-digit CUI (Resolución SAT-DSI-393-2025), so a
  13-digit number is accepted for the `nit` kind.
- What is not validated: whether the CUI exists in RENAP's registry, and the
  literal validity of the department/municipality for people registered abroad
  or naturalised, whose last four digits may not follow the INE codes. Use
  `validate` only as a format and check-digit filter.
