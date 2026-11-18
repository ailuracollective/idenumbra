# idenumbra-mex

Mexico identification validator.

## Documents

- CURP (18 characters).
- RFC (12 characters for companies, 13 for persons).

## Validation algorithm

CURP: `[A-Z]{4} + YYMMDD + sex + state(2) + consonants(3) + homoclave + check
digit`. The birth date must be a real calendar date (the 17th character picks
the century: digit 0-9 = born up to 1999, letter = born from 2000), the sex must
be `H`/`M` (or `X`: RENAPO issues non-binary CURPs and the gob.mx form sends
"No binario" as `X`), the three consonant positions must hold consonants (`X`
when a name has none), the state must be a valid CURP code, and the check digit
is `(10 - Σ value(c) × (18 - i) mod 10) mod 10` over the first 17 characters,
using the public table `0123456789ABCDEFGHIJKLM&OPQRSTUVWXYZ` (`&` stands in for
the Ñ slot; an issued CURP never contains Ñ).

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

- **Official** — DOF 23/10/1996, Acuerdo with the CURP structure and the Anexo I
  state table (`QT` for Querétaro is called out explicitly, to avoid confusion
  with `QR` Quintana Roo): https://dof.gob.mx/nota_detalle.php?codigo=4903440&fecha=23/10/1996
- **Official** — SEGOB, *Instructivo Normativo para la asignación de la CURP*
  (DOF 18/10/2021), positions 1-18, `NE`, and the 0-9 / A-J century
  differentiator: https://www.gob.mx/cms/uploads/attachment/file/337251/Instructivo_Normativo_para_la_Asignacion_de_la_CURP.pdf
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
- **Official** — SAT, *Especificaciones técnicas* for RFC exchange: RFC is 13
  positions for persons and 12 for companies, format `AAAAaammddHHD` with
  `AAAA` = letters (Ñ and `&` allowed), `HH` = homonimia, `D` = check digit:
  https://www.sat.gob.mx/cs/Satellite?blobcol=urldata&blobkey=id&blobtable=MungoBlobs&blobwhere=1461174992692&ssbinary=true
- **Official** — SAT, CFDI de nómina fill-in guide: `Rfc` must be 13 positions
  for a person and 12 for a company:
  https://www.sat.gob.mx/cs/Satellite?blobcol=urldata&blobkey=id&blobtable=MungoBlobs&blobwhere=1461173358253&ssbinary=true
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
- **10-character RFC**: there is no fiscal 10-position RFC. The SAT specs require
  12/13 positions, so the truncated "RFC sin homoclave" of some legacy systems is
  rejected with `InvalidLength`.
- **Check digits**: the official documents above describe the position and its
  purpose but do not print the weighted table. The values used here are the
  long-standing public algorithms, cross-checked against python-stdnum, and the
  test values were generated from them.
- **Not checked**: the CURP list of inconvenient words (Anexo 2), future birth
  dates (the crate has no clock), and the RFC century (dates are read as 20xx,
  which only changes 29/02 when `YY` is `00`).

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
idenumbra = { version = "0.1", features = ["mex"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("MEX", "12345678")?;
```

## Test numbers (synthetic, not real)

- CURP: `BOXW310820HNERXN09` (python-stdnum example), `GALA850315XDFRRN07`
  (non-binary sex `X`), `GALA040229XDFRRN07` (born 2004, letter homoclave)
- RFC (SAT documented example): `EKU9003173C9`
- RFC (synthetic person): `GODE560101AB0`
- RFC with `Ñ`/`&` in the name part: `ÑABC010101ABA`, `A&BC010101CD9`

These values are **synthetic** and do **not** correspond to any real person or
company; the CURPs and generic RFCs were generated locally with this crate's
algorithm. `EKU9003173C9` is the example RFC published by the SAT in its own
guides (Escuela Kemper Urgate), reproduced here only as a documented public
vector.

## Notes

- Feature flag: `mex`.
- Country code (ISO 3166-1 alpha-3): `MEX`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- RFC dates are checked as 20xx because the SAT does not encode the century.
