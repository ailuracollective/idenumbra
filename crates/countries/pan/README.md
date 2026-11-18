# idenumbra-pan

Panama identification validator.

## Documents

- Cédula / RUC (hyphenated parts, or a plain 7-13 digit number).
- RUC NT, the DGI tax number for resident foreigners and entities without
  legal personality (`province-NT-folio-asiento`).
- Passport (6-12 alphanumerics).

## Validation algorithm

Hyphenated numbers are split into three parts: the first is a rollo/tomo
(1-9 digits) or a province (`01-13`) followed by `E`, `N`, `PE`, `AV` or `PI`;
the second part has 1-4 digits and the third 1-9 digits. Four-part values
`province-NT-folio-asiento` are accepted as RUC NT. Plain numbers are accepted
with 7-13 digits.

The Tribunal Electoral publishes no check digit for the cédula. The DGI does
issue a two-digit DV per RUC, and its value depends on the taxpayer type
(persona natural, jurídica, natural NT, jurídica NT, finca) and, for natural
persons, on the province and marker; the DGI's own page says "el sistema [lo]
asigna de manera automática a cada RUC" and that "su dígito verificador cambia
al momento que se inscribe como contribuyente". The DGI's electronic-invoice
*Ficha Técnica* nevertheless refers to "el algoritmo del RUC publicado por la
DGI" and exposes a WS *Consulta de RUC y DV*. The algorithm document itself
("Algoritmo para el cálculo del dígito verificador de la RUC y recibo") could
not be opened in this session — the copies found are behind blocked hosts — so
this crate does not compute the DV: a type-dependent DV is a registry attribute,
and applying an unverified table would reject valid numbers. Two independent
implementations reproduce the algorithm (see below), but both need the taxpayer
type, which this crate's API does not carry. Use the DGI eTax 2.0 *Dígito
Verificador* service.

The same *Ficha Técnica* sizes the RUC field at **5-21 alphanumeric**
characters and the DV at 2 digits. This crate validates only the documented
printed shapes (province/marker-folio-asiento, `NT`, and plain 7-13 digits), so
an alphanumeric RUC outside those shapes is a known false negative.

### Editable constants

- `MAX_PROVINCE_CODE = 13` (`src/validators/validate.rs`) — provinces and
  comarcas `01-13`: 1 Bocas del Toro, 2 Coclé, 3 Colón, 4 Chiriquí, 5 Darién,
  6 Herrera, 7 Los Santos, 8 Panamá, 9 Veraguas, 10 Guna Yala, 11
  Emberá-Wounaan, 12 Ngäbe-Buglé, 13 Panamá Oeste (created in 2014 out of
  province 8).
- `ID_MARKERS = ["E", "N", "PE", "AV", "PI"]` (`src/validators/validate.rs`) —
  markers written after the province code (or instead of it): `E` extranjero,
  `N` naturalizado, `PE` panameño nacido en el exterior, `AV` antes de la
  vigencia, `PI` población indígena.
- `NT_TAG = "NT"` (`src/validators/validate.rs`) — DGI tax-number tag.

The `EE` (transitory foreigner) and `SB` (pre-1953 San Blas) series are **not**
accepted, and neither is an `E-NT` prefix. Decreto 53/1958 art. 9 — still the
rule that defines the numbering — lists exactly four shapes: province code,
`PE`, `N` and `E`. No later norm, DGI form or OECD TIN profile adds `EE`, `SB`
or `E-NT`: the DGI's RUC form has a single `N° de Cédula o N.T` field, and an
NT is written `province-NT-folio-asiento`. `EE` and `SB` survive only in
third-party compilations, so they stay rejected until the Tribunal Electoral
publishes their shape.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — Tribunal Electoral, Dirección Nacional de Cedulación:
  https://www.tribunal-electoral.gob.pa/direccion-nacional-de-cedulacion/la-cedula
  — checked 2026-11-17 (the site answers `403` to automated clients and was read
  through its archived copy of the cedulación rules)
- **Official** — Tribunal Electoral, cedulación rules (Decreto No. 53 de
  29/07/1958, art. 9: province code + tomo + asiento, `PE-`, `N-`, `E-`):
  https://web.archive.org/web/20100728085351/http:/www.tribunal-electoral.gob.pa/cedulacion/index.html
  — checked 2026-11-17
- **Official** — DGI, *Ficha Técnica de la Factura Electrónica* (WS
  `feConsRucDV`, field `dRuc` 5-21 alphanumeric, `dDV` 2 digits, "Debe ser
  calculado en base al algoritmo del RUC publicado por la DGI"):
  https://dgi.mef.gob.pa/_7FacturaElectronica/source/4-Anexo-3-Ficha-T%C3%A9cnica-Factura-Electr%C3%B3nica-Proveedores-Autorizaci%C3%B3n-Calificados-V1.0.pdf
  — checked 2026-11-17
- **Official** — DGI, *Dígito Verificador (D.V)*: the DV is assigned per RUC
  and per taxpayer type (Natural, Jurídica, Natural NT, Jurídica NT, Finca) and
  changes on registration: https://dgi.mef.gob.pa/DV — checked 2026-11-17
- **Official** — DGI, eTax 2.0 *Consultar DV* (the only DV oracle):
  https://etax2.mef.gob.pa/etax2web/Ruc/ConsultarDV.aspx — checked 2026-11-17
- **Official** — DGI, RUC registration form: one identification field,
  `N° de Cédula o N.T` (no `EE`, `SB` or `E-NT` field/prefix):
  https://dgi.mef.gob.pa/Form-Tramites/FormulariosPdf/Formulario-RUC.pdf
  — checked 2026-11-17
- **Official (international)** — OECD, *Panama TIN* (RUC = cédula for natural
  persons, 2-digit DV, `NT` prefix for foreigners, RUC NT example `3-NT-3-33`):
  https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/panama-tin.pdf
  — checked 2026-11-17
- **Implementation** — https://github.com/apple314159/panama-dv (Python: the
  full two-round DV algorithm with the taxpayer-type cross-reference table,
  citing the ANIP document *Algoritmo para el cálculo del dígito verificador de
  la RUC y recibo*; the NT branch is marked TODO)
- **Implementation** — https://github.com/leira2985/panama-ruc (TypeScript: the
  same DV, and its API needs a `typeHint` to disambiguate NT and short RUCs —
  evidence that the DV is not a function of the number alone)
- **Implementation** — province codes and marker meanings, collected from
  Tribunal Electoral forms and press: https://oobrien.com/2017/05/panamas-population-geographies
  — checked 2026-11-17
- **Implementation** — https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Panam%C3%A1)
  — checked 2026-11-17

The Tribunal Electoral publishes no machine-readable catalogue of province
codes or marker meanings; the table above comes from its own forms, its 1958
cedulación rules and its press.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["pan"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("PAN", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `8-123-4567`, `PE-12-3456`, `8AV-123-4567`
- RUC (juridical): `2588017-1-831938` (without DGI check digits)
- RUC NT: `8-NT-1-13656`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally matching the document structure. Source: DGI RUC structure (OECD *Panama TIN* for the `NT` shape) and the Tribunal Electoral numbering rules from Decreto 53/1958.

## Notes

- Feature flag: `pan`.
- Country code (ISO 3166-1 alpha-3): `PAN`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The DGI check digits (two digits, type-dependent) are not checked; use the
  DGI eTax 2.0 `DV` service for that.
- `validate_with("id_card" | "ruc" | "passport")` forces the kind. `id_card`
  rejects the four-part NT (a tax number, not an identity card); `ruc` accepts
  the cédula (a natural person's RUC is their cédula) and the NT; neither
  accepts a passport.
- `EE` and `SB` series are rejected: no issuing rule defines them (Decreto
  53/1958 art. 9 lists only province, `PE`, `N` and `E`).
