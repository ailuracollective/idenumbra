# idenumbra-mex

Mexico identification validator.

## Documents

- CURP (18 characters).
- RFC (12 characters for companies, 13 for persons).

## Validation algorithm

CURP: `[A-Z]{4} + YYMMDD + [HM] + state(2) + consonants(3) + homoclave + check
digit`. The birth date must be a real calendar date (the 17th character picks
the century: digit = 19xx, letter = 20xx), the gender must be `H`/`M`, the state
must be a valid CURP code, and the check digit is
`(10 - Σ value(c) × (18 - i) mod 10) mod 10` over the first 17 characters.

RFC: 3 letters (companies) or 4 letters (persons) plus `YYMMDD` plus 3
alphanumerics; the check character is `(11 - Σ value(c) × (13 - i)) mod 11`,
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

- **Official** — DOF 23/10/1996, Acuerdo with the CURP state code table:
  https://dof.gob.mx/nota_detalle.php?codigo=4903440&fecha=23/10/1996
- **Official** — SEGOB, *Instructivo Normativo de la CURP* (2006), Anexo 4
  (`NE` for births abroad): http://www.ordenjuridico.gob.mx/Federal/PE/APF/APC/SEGOB/Instructivos/InstructivoNormativo.pdf
- **Official** — RENAPO, *Manual técnico Plataforma Única de Identidad*, Anexo 5
  (2026): https://dof.gob.mx/2026/GOBERNACION/SG_230126_VES_02.pdf
- **Official** — RMF 2026 (DOF 28/12/2025): generic RFC rule 2.7.1.23 and RFC
  length rule 2.13.3: https://dof.gob.mx/2025/SHCP/SHCP_281225_01.pdf
- **Official** — RENAPO CURP portal: https://www.gob.mx/curp/
- **Official** — SAT, *Anexo 20* fill-in guide:
  http://omawww.sat.gob.mx/tramitesyservicios/Paginas/documentos/Anexo_20_Guia_de_llenado.pdf
- **Implementation** — python-stdnum `mx.curp` and `mx.rfc` (the SAT publishes
  no check-digit algorithm): https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/mx/curp.py

The 2026 RENAPO manual writes `QO` for Querétaro and adds `XX`; `QT` is the
canonical code and `XX` is not part of an issued CURP, so neither change is
applied here. The SAT publishes no check-digit algorithm.

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

- CURP: `BOXW310820HNERXN09`
- RFC (SAT test): `EKU9003173C9`
- RFC (synthetic person): `GODE560101AB0`

These values are **synthetic** and do **not** correspond to any real person or company. `EKU9003173C9` is the SAT's documented test RFC (Escuela Kemper Urgate). Source: SAT documentation and python-stdnum examples.

## Notes

- Feature flag: `mex`.
- Country code (ISO 3166-1 alpha-3): `MEX`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- RFC dates are checked as 20xx because the SAT does not encode the century.
- Only 12- and 13-character RFCs are accepted; the truncated 10-character form
  used by some legacy systems is rejected. The CURP name blacklist and
  future birth dates are not checked.
