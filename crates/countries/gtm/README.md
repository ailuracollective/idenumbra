# idenumbra-gtm

Guatemala identification validator.

## Documents

- CUI/DPI (13 digits, plus the 12-digit form SAT still describes) — full: check
  digit plus department and municipality.
- NIT (2-12 characters, last one `0-9` or `K`; spaces, dots and the separator
  before the check character are dropped) — full.
- Passport (6-12 alphanumerics) — structure only.

## Validation algorithm

CUI: layout `correlativo(8) + check digit + departamento(2) + municipio(2)`.
SAT's FEL technical annex, *Reglas y validaciones* 2.1 (October 2026), §5.1.4
states that the check digit is computed from the seven or eight correlativo
digits, that the last four digits never take part, and that they are multiplied
by weights 9 to 2, the products summed, the sum multiplied by 10 and taken
`mod 11`. The annex is the electronic-invoice rulebook, but it validates the
canonical CUI/NIT themselves — §2.2.4 numeral 3 makes SAT reject a document
whose CUI has an invalid check digit — so the rule describes the identifier, not
only an invoice field. The annex prints the position labels but not the coefficients; label
+ 1 is the set every implementation uses, and with it the rule is algebraically
`sum(first 8 digits × 2..9) mod 11`, which this crate computes. When the residue
is `10` no digit satisfies the rule, so the value is rejected with
`UnrepresentableCheckDigit`; the annex defines no `10 -> 0` mapping, and SAT's
own FEL validator compares against the same digit. A CUI printed with `0` there
would be a false negative (about one body in eleven).

The annex also allows a CUI of 12 digits (§5.1.4.1): the same layout with the
correlativo's leading zero dropped. `validate_with("cui" | "dpi")` accepts it by
left-padding that zero. A 12-digit number is also a valid NIT shape, so
`validate` tries the more specific CUI layout first and falls back to the NIT
rule.

§5.1.1 and §5.1.2 also tell issuers how to *depurate* what the user types:
spaces and dots are dropped anywhere, the hyphen or underscore right before the
check character is dropped, and a lowercase `k` becomes `K`. The CUI is normally
printed as `AAAA BBBBB CCCC`, so both entry points apply that depuration before
classifying. A separator anywhere else is still rejected.

The department must be `01-22` and the municipality must exist for that
department (`01` up to the department's count). §5.1.5 puts the department
before the municipality and prints a table of counts; this crate uses INE's
current 340 municipalities, which keeps 33 for Huehuetenango (SAT's 2.1 edition
already has Sipacate's 14 for Escuintla but still prints 32).

Special registrations do not need a special code. Decreto 39-2010 art. 1
(reforming art. 4 of Decreto 90-2005) keeps the 13-digit CUI and says that for
"inscripciones consulares, inscripciones por naturalización o de residentes
extranjeros u otras" the department and municipality boxes are used "según lo
establezca el Directorio del RENAP". Acuerdo de Directorio 55-2014 (and 36-2023
for minors) assigns a naturalized Guatemalan or domiciled foreigner the code of
the department and municipality **where the inscription was made** — an ordinary
`01-22` / municipality pair. So the municipality table can legitimately reject a
nonexistent code; what it cannot do is reject a valid registration place. The
consular case (a birth registered abroad, where no municipality of birth
exists) is not published and is listed under Notes.

NIT: each body digit is multiplied by a factor that starts at `body length + 1`
and decreases; the check character is `(11 - total mod 11) mod 11`, where `11`
is written as `0` and `10` as `K` (§5.1.3). §5.1.1 tells issuers to accept the
hyphen people type right before the check character and drop it, so
`1234567-9` is accepted and a hyphen anywhere else is not.

Since 2023-10-04 a natural person's NIT is the first 9 digits of the CUI (SAT
FAQ: Guatemalans and domiciled foreigners only; earlier NITs did not change),
and the full 13-digit CUI is also the NIT after the Corte de Constitucionalidad
ruling on Resolución SAT-DSI-393-2025, in force since 2026-09-23. A 9-digit CUI
prefix passes the NIT check because the CUI and NIT weights are mirrors: the CUI
digit equals the NIT digit computed over the same eight digits, except that the
CUI can never print a `10`. `validate_with(.., "nit")` checks a 13-digit value
as a CUI.

### Editable constants

- `MUNICIPALITIES_PER_DEPARTMENT` (`src/validators/cui.rs`) — municipality
  count per department, indexed from `01` (Guatemala); index `0` is unused, so
  department `00` is invalid. The table sums 340. Escuintla gained Sipacate
  (`14`, Decreto 4-2015, code `0514`) and Huehuetenango gained Petatán (`33`,
  Decreto 6-2015, code `1333`); keep the count in sync when another municipality
  is created. No municipality was created after Petatán: in June 2026 El Cerinal
  was still waiting for initiative 5558, which would make it number 341.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.


Official:

- SAT, *Reglas y validaciones* (Régimen FEL), versión 2.1, octubre 2026 —
  §5.1.1 NIT depuration (spaces, dots, hyphen before the check character), §5.1.3
  NIT of 2 to 12 characters with check character `0-9`/`K`, §5.1.4 CUI of 12 or
  13 digits, check digit = products × 10 mod 11 with the coefficients not
  printed, §5.1.5 department then municipality and the table of counts (the
  annex notes the table "podrá ser actualizada" when a municipality is created);
  §2.2.4 numeral 3 makes SAT itself reject a CUI whose check digit is invalid:
  https://portal.sat.gob.gt/portal/descarga/14852/reglas-y-validaciones/97423/fel_reglas_y_validaciones-3 —
  checked 2026-11-17 (the portal blocks automated clients with Cloudflare; the
  PDF text was read through a text-extraction proxy)
- SAT, *Preguntas frecuentes — Información de Contribuyentes* (from 04/10/2023 a
  NIT is the first 9 digits of the CUI; only for Guatemalans and domiciled
  foreigners; earlier NITs unchanged):
  https://portal.sat.gob.gt/portal/preguntas-frecuentes/informacion-de-contribuyentes/ —
  checked 2026-11-17
- SAT, *¿Qué significa para el guatemalteco la armonización del CUI con el NIT?*
  (2025-03-31, Resolución SAT-DSI-393-2025):
  https://portal.sat.gob.gt/portal/noticias/que-significa-para-el-guatemalteco-la-armonizacion-del-cui-con-el-nit —
  checked 2026-11-17
- RENAP, *¿Qué es el CUI?* (13 digits; the last four are the department and then
  the municipality of birth): https://www.renap.gob.gt/noticias/que-es-el-cui —
  checked 2026-11-17
- Congreso de la República, Decreto 39-2010, art. 1 (13 digits = 8 + 1 verifier
  + 2 department + 2 municipality; consular, naturalization and foreign-resident
  registrations use those boxes "según lo establezca el Directorio del RENAP"):
  https://www.renap.gob.gt/sites/default/files/informacion-publica/1_decreto_39-2010_reformas_a_la_ley_del_registro_nacional.pdf
  — checked 2026-11-17
- RENAP, Acuerdo de Directorio 55-2014 ("A los guatemaltecos naturalizados y
  extranjeros domiciliados, les corresponde en el CUI el código del departamento
  y municipio donde fue efectuada la inscripción"):
  https://www.renap.gob.gt/sites/default/files/informacion-publica/Acuerdo-de-Directorio-55-2014-58-2014-y-59-2014.pdf
  — checked 2026-11-17 (scanned, read through the search index; the same rule
  appears in Acuerdo 36-2023 for minors)
- RENAP, Acuerdo de Directorio 36-2023 (minors: naturalized Guatemalans and
  domiciled foreigners get the department and municipality of the inscription):
  https://www.renap.gob.gt/sites/default/files/informacion-publica/acuerdo_de_directorio_36-2023_dpim.pdf
  — checked 2026-11-17 (scanned, read through the search index)
- RENAP, *¿Qué es el DPI y por qué tengo CUI?* (8 digits assigned by RENAP, one
  check digit, department, municipality):
  https://www.renap.gob.gt/noticias/que-es-el-dpi-y-por-que-tengo-cui — checked
  2026-11-17
- SAT, *Consulta CUI / NIT*: https://portal.sat.gob.gt/portal/consulta-cui-nit —
  checked 2026-11-17
- INE, *Estimaciones y proyecciones de población a nivel departamental y
  municipal* (22 departments, 340 municipalities):
  https://www.ine.gob.gt/ine/wp-content/uploads/2020/08/02-Presentacion-de-Resultados-Departamentales-y-Municipales.pdf —
  checked 2026-11-17
- Congreso de la República, Decreto 6-2015 (creates Petatán, Huehuetenango):
  https://www.congreso.gob.gt/assets/uploads/info_legislativo/decretos/2015/06-2015.pdf —
  checked 2026-11-17
- SEGEPLAN, *PDM-OT Sipacate, Escuintla* (Decreto 4-2015, code `0514`):
  https://portal.segeplan.gob.gt/segeplan/wp-content/uploads/2025/05/0514_PDM_OT_SIPACATE.pdf —
  checked 2026-11-17
- IGN, municipal boundaries layer metadata (codes `0514` and `1333`):
  https://www.arcgis.com/home/item.html?id=6164b3415296419893534831bbe1c8a3 —
  checked 2026-11-17

Secondary (news, used only for dates of regulatory changes):

- La Hora, *CUI en lugar del NIT* (2026-09-25: the CC ruling is in force, the
  full CUI equals the NIT):
  https://lahora.gt/lh-economia/esmith/2026/09/25/cui-en-lugar-del-nit-sat-confirma-que-la-nueva-regla-ya-esta-vigente-en-guatemala-que-cambia —
  checked 2026-11-17
- Prensa Libre, *El Cerinal… municipio 341* (2026-06-14: no new municipality
  approved): https://www.prensalibre.com/guatemala/comunitario/el-cerinal-la-aldea-que-lleva-mas-de-15-anos-esperando-convertirse-en-el-municipio-341-de-guatemala —
  checked 2026-11-17

Cross-checks (not authority):

- python-stdnum `stdnum.gt.nit` (2-12 characters, same mod 11 check digit):
  https://arthurdejong.org/python-stdnum/doc/2.2/stdnum.gt.nit
- https://github.com/aalonzolu/cui-gt (Python and JS: 13 digits only, ordinary
  `01-22` departments, rejects `00`, checksum `Σ d × (i+2) mod 11`; uses the
  old 32 municipalities for Huehuetenango)
- https://github.com/alextello/validador-dpi-nit (same checksum and department
  table, Escuintla 13 and Huehuetenango 32). Together with the previous one,
  every public validator read here rejects department/municipality `00` and
  accepts only ordinary codes, which supports the Decreto 39-2010 reading above.
- Ministerio de Finanzas, `CuiValidator.cs` (weights 2..9 and
  `sum % 11 == check digit`, i.e. no `10 -> 0` mapping):
  https://github.com/minfingt/validators
- Conflicting tools, not followed: https://box4.dev/es/guatemala/validador-cui
  and https://administrativonexusvida.web.app/en/tools/procedures/guatemala-dpi-nit-validator
  map a residue of `10` to `0`. Their rule (`11 - sum mod 11`) and the annex's
  (`sum × 10 mod 11`) are the same arithmetic, so they disagree only in that
  case, and the annex has no digit to compare there.
- https://github.com/alextello/validador-dpi-nit (weights 2..9, `mod 11`) and
  https://github.com/aalonzolu/cui-gt (same formula, but 32 municipalities in
  Huehuetenango, i.e. before Petatán).

## Usage

All features:
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["gtm"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("GTM", "12345678")?;
```

## Test numbers (synthetic, not real)

- CUI/DPI: `1234567890101`
- CUI/DPI in Sipacate, Escuintla: `1234567890514`
- CUI/DPI in Petatán, Huehuetenango: `1234567891333`
- 12-digit CUI (correlativo without its leading zero): `123456790101`, whose
  padded form `0123456790101` is the same number
- NIT: `12345679`
- NIT written with the hyphen: `1234567-9`
- NIT written with dots and the hyphen: `1.234.567-9`
- CUI in its printed form with two spaces: `1234 56789 0101`
- NIT with check character K: `1023456K`
- NIT whose check character is 0: `12345660`
- NIT of a natural person (first 9 CUI digits): `123456789`
- CUI body whose residue is 10, rejected: `1000000700101`

These values are **synthetic** and do **not** correspond to any real person or
company. They were generated locally with this crate's validation algorithm.
The property-based tests generate additional synthetic values locally from the
published check-character algorithms and municipality table.

## Notes

- Feature flag: `gtm`.
- Country code (ISO 3166-1 alpha-3): `GTM`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- `validate_with("cui" | "dpi" | "nit" | "passport")` forces the document type.
  `validate` infers: 13 digits are a CUI; 12 digits are tried as a CUI and then
  as a NIT; other 2-12 digit values, a hyphen before the check character or a
  final `K` are a NIT; anything else alphanumeric is a passport.
- What is not validated: whether the CUI exists in RENAP's registry, and the
  department/municipality code of a birth registered at a consulate abroad. The
  law leaves that code to the RENAP Directorship and no published acuerdo fixes
  it; naturalized people and domiciled foreigners do get an ordinary code (the
  place of registration, Acuerdo 55-2014), which the table accepts. The
  12-digit CUI shares its shape with the NIT; inference tries the CUI layout
  first.
