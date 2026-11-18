# idenumbra-gtm

Guatemala identification validator.

## Documents

- CUI/DPI (13 digits).
- NIT (2-12 digits, check character `0-9` or `K`).
- Passport (6-12 alphanumerics).

## Validation algorithm

CUI: layout `correlativo(8) + check digit + departamento(2) + municipio(2)`.
The check digit is `sum(first 8 digits × 2..9) mod 11`; the department must be
`01-22` and the municipality must exist for that department. NIT: each body
digit is multiplied by a factor starting at `body length + 1` and decreasing;
the check character is `(11 - total mod 11) mod 11`, where `10` is written as
`K`.

### Editable constants

- `MUNICIPALITIES_PER_DEPARTMENT` (`src/validators/cui.rs`) — municipality
  count per department, indexed from `01` (Guatemala). The table sums 340, the
  current INE total. Escuintla gained Sipacate (`0514`, Decreto 4-2015) and
  Huehuetenango gained Petatán (`1333`, Decreto 6-2015); keep the count in
  sync when another municipality is created.

## References

- **Official** — RENAP, Decreto 90-2005 (CUI = 8 correlativo + check digit + 2
  department + 2 municipality): https://www.renap.gob.gt/sites/default/files/contenido-pdf/decreto_90-2005_ley_del_registro_nacional_de_las_personas.pdf
- **Official** — RENAP, *Qué es el CUI*: https://www.renap.gob.gt/noticias/que-es-el-cui
- **Official** — SAT, consulta CUI/NIT: https://portal.sat.gob.gt/portal/consulta-cui-nit
- **Official** — SEGEPLAN, *Plan de Desarrollo Departamental Escuintla* (14
  municipalities after Sipacate, Decreto 4-2015): https://portal.segeplan.gob.gt/segeplan/wp-content/uploads/2024/02/05_PDD_ESCUINTLA-.pdf
- **Official** — SEGEPLAN, *Plan de Desarrollo Departamental Huehuetenango* (33
  municipalities after Petatán, Decreto 6-2015): https://portal.segeplan.gob.gt/segeplan/wp-content/uploads/2024/12/13_PDD_HUEHUETENANGO.pdf
- **Implementation** — https://github.com/alextello/validador-dpi-nit
- **Implementation** — https://es.wikipedia.org/wiki/Documento_Personal_de_Identificaci%C3%B3n

RENAP/SAT do not publish the CUI or NIT check-digit algorithms; the weights used
here are the community standard. The department/municipality counts are official:
INE lists 22 departments and 340 municipalities.

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
idenumbra = { version = "0.1", features = ["gtm"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("GTM", "12345678")?;
```

## Test numbers (synthetic, not real)

- CUI/DPI: `1234567890101`
- CUI/DPI in Sipacate, Escuintla: `1234567890514`
- NIT: `12345679`
- NIT with check character K: `1023456K`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SAT CUI/NIT rules.

## Notes

- Feature flag: `gtm`.
- Country code (ISO 3166-1 alpha-3): `GTM`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- `validate_with("cui" | "dpi" | "nit" | "passport")` forces the document type.
- Since 2026 the DPI/CUI number is also the NIT for natural persons, so a
  13-digit number is accepted for the `nit` kind.
