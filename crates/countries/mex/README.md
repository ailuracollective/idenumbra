# idenumbra-mex

Mexico identification validator.

## Documents

- CURP (18 characters).
- RFC (12 characters for companies, 13 for persons).

## Validation algorithm

CURP: `[A-Z]{4} + YYMMDD + sex + state(2) + consonants(3) + homoclave + check
digit`. The birth date must be a real calendar date (the 17th character picks
the century: digit `0`-`9` = born up to 1999, letter `A`-`J` = born from 2000),
the sex must be `H`/`M` (or `X`: RENAPO issues non-binary CURPs and the gob.mx
form sends "No binario" as `X`), the three consonant positions must hold
consonants (`X` when a name has none), the state must be a valid CURP code, and
the check digit is `(10 - Σ value(c) × (18 - i) mod 10) mod 10` over the first 17
characters, using the public table
`0123456789ABCDEFGHIJKLMN&OPQRSTUVWXYZ` (`&` stands in for the Ñ slot; an issued
CURP never contains Ñ).

RFC: 3 letters (companies) or 4 letters (persons) plus `YYMMDD` plus 3
alphanumerics; the check character is `(11 - Σ value(c) × (13 - i)) mod 11`
over the first 11/12 characters, padded on the left with a space for companies,
where `10` is written as `A`. `Ñ` and `&` are accepted in the name part. The
SAT generic RFCs `XAXX010101000` and `XEXX010101000` are accepted as-is.

### Editable constants

- `CURP_STATES` (`src/validators/curp.rs`) — the 31 state codes plus `DF` and
  `NE` ("foráneo", birth registered abroad; `AS` is Aguascalientes). Edit only
  if a state is created or renamed; `XX` (unknown) is deliberately not accepted.
- `GENERIC_RFCS` (`src/validators/rfc.rs`) — `XAXX010101000` when the taxpayer
  has no RFC (public at large) and `XEXX010101000` for a resident abroad not
  registered in the RFC.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — DOF 23/10/1996, Acuerdo with the CURP structure and the Anexo I
  state table (`QT` for Querétaro is called out explicitly, to avoid confusion
  with `QR` Quintana Roo): https://dof.gob.mx/nota_detalle.php?codigo=4903440&fecha=23/10/1996
- **Official** — DOF 13/06/2018, *Instructivo Normativo para la asignación de la
  CURP* (base text; position 11 = "H para hombre y M para mujer", position 17 =
  `0-9` hasta 1999 / `A-J` desde 2000, position 18 = verification digit):
  https://dof.gob.mx/nota_detalle_popup.php?codigo=5526717 — checked 2026-11-17
- **Official** — SEGOB, *Instructivo Normativo para la asignación de la CURP*
  (DOF 18/10/2021, consolidated text), positions 1-18, `NE`, and the 0-9 / A-J
  century differentiator: https://www.gob.mx/cms/uploads/attachment/file/337251/Instructivo_Normativo_para_la_Asignacion_de_la_CURP.pdf
- **Official** — SEGOB, *Memoria Documental 2018-2024, Política de Registro de
  Población*: records the "No Binario" option and the `X` "en la posición número
  11 de la CURP, correspondiente al sexo":
  https://www.gob.mx/cms/uploads/attachment/file/944642/3_MD_POL_TICA_DE_REGISTRO_DE_POBLACI_N_compressed.pdf
  — checked 2026-11-17 (the PDF is image-heavy; the quoted sentence was read
  through the search index)
- **Official** — RENAPO, *Reglas para la ejecución de los procedimientos para la
  asignación de la CURP* (22/10/2021), character 17: 0-9 up to 1999, A-J from
  2000: https://www.gob.mx/cms/uploads/attachment/file/681698/reglas_para_la_ejecucion_de_los_procedimientos_asignacion_de_la_curp.pdf
- **Official** — SEGOB, manual of the *Plataforma Única de Identidad* (DOF
  Edición Vespertina 23/01/2026), Anexo 5: maps `NE` to FORÁNEO and includes
  `XX` = DESCONOCIDO while stating that `XX` "no forma parte de la CURP
  oficial"; the table prints `QO` for Querétaro: https://dof.gob.mx/2026/GOBERNACION/SG_230126_VES_02.pdf
- **Official** — RMF 2026 (DOF 28/12/2025), rule 2.7.1.23: generic RFCs
  `XAXX010101000` / `XEXX010101000`:
  https://www.sat.gob.mx/minisitio/NormatividadRMFyRGCE/documentos2026/rmf/compiladas/Compilado_Primera_Modificacion_a_la-Resolucion_Miscelanea_Fiscal_para_2026.pdf
- **Official** — SAT, *Especificaciones técnicas* for RFC exchange (February
  2021), section 6: RFC is 13 positions for persons and 12 for companies, format
  `AAAAaammddHHD` with `AAAA` = letters (Ñ and `&` allowed), `HH` = homonimia,
  `D` = check digit:
  https://www.sat.gob.mx/cs/Satellite?blobcol=urldata&blobkey=id&blobtable=MungoBlobs&blobwhere=1461174992692&ssbinary=true
- **Official** — SAT, CFDI de nómina fill-in guide (10/11/2017, CFDI 3.3):
  `Rfc` must be 13 positions for a person and 12 for a company:
  https://www.sat.gob.mx/cs/Satellite?blobcol=urldata&blobkey=id&blobtable=MungoBlobs&blobwhere=1461173358253&ssbinary=true
- **Official** — SEGOB, *Instructivo Normativo para la asignación de la CURP*
  (March 2006, superseded), Anexo 2 with the tabulated catalogue of inconvenient
  words and the position-17 wording "A-Z":
  http://ordenjuridico.gob.mx/Federal/PE/APF/APC/SEGOB/Instructivos/InstructivoNormativo.pdf
- **Official** — RENAPO CURP portal: https://www.gob.mx/curp/
- **Implementation** — python-stdnum `mx.curp` and `mx.rfc`:
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/mx/curp.py and
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/mx/rfc.py

Open questions resolved conservatively (checked 2026-11-17):

- **`QT` vs `QO`**: the 1996 Acuerdo (construction rule) and the 2021 Instructivo
  use `QT`; the 2026 PUI manual prints `QO` in an API mapping table. Only `QT` is
  accepted.
- **`XX`**: the same 2026 manual says `XX` is not part of an issued CURP, so it is
  rejected. `NE` (born abroad) is accepted.
- **Sex `X`**: accepted. The 2026 PUI manual defines the sex value as "H para
  hombres, M para mujeres y X para el resto de los casos", the gob.mx CURP form
  offers "No binario", and RENAPO issues non-binary CURPs with `X` at position
  11. The 2021 Instructivo still prints only H/M.
- **Position 17 is `0`-`9` or `A`-`J`**: enforced. The Instructivo (position 17)
  and the Reglas ("Carácter 17") both say `0`-`9` for births up to 31/12/1999 and
  `A`-`J` from 01/01/2000; the `A`-`J` range is the homonimia indicator in base
  10 (`A` = 0). The 2006 Instructivo printed "A-Z", which is where the wider
  range comes from, but that text was replaced in 2018 and last amended in 2021.
  A letter outside `A`-`J` is not an issued CURP today, so `K`-`Z` are rejected
  even when the check digit closes. Residual risk: a CURP issued under the 2006
  wording with more than nine homonyms could carry `K`-`Z`; no such value has
  been reported, and the current rule is `A`-`J`.
- **10-character RFC**: there is no fiscal 10-position RFC. The SAT specs require
  12/13 positions, so the truncated "RFC sin homoclave" of some legacy systems is
  rejected with `InvalidLength`.
- **Ñ and `&`**: accepted in the letter part of an RFC. The SAT *Especificaciones
  Técnicas* (February 2021) still list `[A-Z (incluso Ñ) y &]` there; the RMF
  2026 does not mention them, and no later SAT document we found changes the
  alphabet.
- **Check digits**: the official documents above describe the position and its
  purpose but do not print the weighted table. The values used here are the
  long-standing public algorithms, cross-checked against python-stdnum, and the
  test values were generated from them.
- **Not checked**: future birth dates (the crate has no clock) and the RFC
  century (dates are read as 20xx, which only changes 29/02 when `YY` is `00`).
- **Inconvenient-word catalogue (not applied)**: the Reglas 2021 announce an
  "Anexo 01. Catálogo de palabras altisonantes" but do not print it in the
  published document; the only tabulated list we found is Anexo 2 of the 2006
  Instructivo, a text the 2018/2021 Instructivo replaced. The rule replaces the
  second letter of positions 1-4 with `X` when they spell a listed word, so it
  governs generation, and the 1996 Acuerdo (the origin of the key) contains no
  such rule, so CURPs issued before the catalogue existed, or before a word was
  added to it, legitimately start with one. Rejecting them would be a false
  negative on a document RENAPO did issue, so nothing is checked (python-stdnum
  rejects them with the 2006 list; this crate deliberately does not).

Official links checked on 2026-11-17; the implementation links are public
cross-checks, not dated.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["mex"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("MEX", "12345678")?;
```

## Test numbers (synthetic, not real)

- CURP: `GALA850315HNERXN05` (born abroad, state `NE`),
  `GALA850315XDFRRN07` (non-binary sex `X`), `GALA040229XDFRRNA7` (born 2004,
  letter homoclave `A`)
- RFC (person): `GODE560101AB0`
- RFC (company): `ABC010101AB1`
- RFC with `Ñ`/`&` in the name part: `ÑABC010101ABA`, `A&BC010101CD9`

These values are **synthetic** and do **not** correspond to any real person or
company; the CURPs and RFCs were generated locally with this crate's algorithm.
The SAT generic RFCs `XAXX010101000` and `XEXX010101000` (RMF 2026, rule
2.7.1.23) are used as-is; `XAXX010101000` does not even close its check digit,
which is why both are accepted before the checksum.

## Notes

- Feature flag: `mex`.
- Country code (ISO 3166-1 alpha-3): `MEX`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- RFC dates are checked as 20xx because the SAT does not encode the century.
