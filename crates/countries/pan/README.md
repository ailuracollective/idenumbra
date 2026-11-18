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

The Tribunal Electoral publishes no check digit for the cédula, and the DGI
check digit cannot be computed from the number. The DGI's own page says the DV
"el sistema [lo] asigna de manera automática a cada RUC" per taxpayer type
(natural, juridical, natural NT, juridical NT, finca) and that "su dígito
verificador cambia al momento que se inscribe como contribuyente" — a DV held
before registration changes on registration:

- the DGI issues **two** digits per RUC, and their value depends on the
  taxpayer type (persona natural, jurídica, natural NT, jurídica NT, finca)
  and, for natural persons, on the province and marker;
- it is a registry attribute rather than a function of the number;
- the algorithm is not published; the DGI looks the DV up in eTax 2.0.

So the DV is documented but deliberately not validated here. Use the DGI eTax
2.0 *Dígito Verificador* service for it.

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

- **Official** — Tribunal Electoral, Dirección Nacional de Cedulación:
  https://www.tribunal-electoral.gob.pa/direccion-nacional-de-cedulacion/la-cedula
  — checked 2026-11-17 (the site answers `403` to automated clients and was read
  through its archived copy of the cedulación rules)
- **Official** — Tribunal Electoral, cedulación rules (Decreto No. 53 de
  29/07/1958, art. 9: province code + tomo + asiento, `PE-`, `N-`, `E-`):
  https://web.archive.org/web/20100728085351/http:/www.tribunal-electoral.gob.pa/cedulacion/index.html
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
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["pan"] }
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
- `EE` and `SB` series are rejected: no issuing rule defines them (Decreto
  53/1958 art. 9 lists only province, `PE`, `N` and `E`).
