# idenumbra-pan

Panama identification validator.

## Documents

- Cédula / RUC (hyphenated parts, or a plain 7-13 digit number).
- Passport (6-12 alphanumerics).

## Validation algorithm

Hyphenated numbers are split into three parts: the first is a rollo/tomo
(1-9 digits) or a province (`01-13`) followed by `E`, `N`, `PE`, `AV` or `PI`;
the second part has 1-4 digits and the third 1-9 digits. Plain numbers are
accepted with 7-13 digits. The DGI RUC check digits depend on the taxpayer
type (natural, juridical, NT, finca) and cannot be derived from the number
alone, so they are documented but not validated.

### Editable constants

- `MAX_PROVINCE_CODE = 13` (`src/validators/validate.rs`) — provinces `01-13`;
  `13` is Panama Oeste, created in 2014 out of province `8`.
- `ID_MARKERS = ["E", "N", "PE", "AV", "PI"]` (`src/validators/validate.rs`) —
  markers written after the province code (or instead of it): `E` extranjero,
  `N` naturalizado, `PE` panameño nacido en el exterior, `AV` antes de la
  vigencia, `PI` población indígena. The `EE` (transitory foreigner), `NT`
  (temporary tax ID) and `SB` (pre-1953 San Blas) series are **not** accepted
  yet; add them here if you need them.

## References

- **Official** — Tribunal Electoral, Dirección Nacional de Cedulación: https://www.tribunal-electoral.gob.pa/direccion-nacional-de-cedulacion/la-cedula
  (the site blocks automated clients; consult it in a browser)
- **Official** — DGI, RUC check digit: https://dgi.mef.gob.pa/DV
- **Implementation** — province codes and markers, from a Tribunal Electoral
  bulletin: https://oobrien.com/2017/05/panamas-population-geographies
- **Implementation** — https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Panam%C3%A1)

The Tribunal Electoral publishes no machine-readable catalogue of province
codes or marker meanings; the values above come from its own forms and press.

Official links were opened on 2026-11-17 (the Tribunal Electoral site answers
`403` to automated clients; open it in a browser); the implementation links are
public cross-checks, not dated.

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

- Cédula: `8-123-4567`
- RUC (juridical): `2588017-1-831938` (without DGI check digits)

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally matching the document structure. Source: DGI RUC structure and documentation.

## Notes

- Feature flag: `pan`.
- Country code (ISO 3166-1 alpha-3): `PAN`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The DGI check digits (two digits, type-dependent) are not checked; use the
  DGI `DV` service for that.
