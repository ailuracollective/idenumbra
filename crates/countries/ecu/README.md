# idenumbra-ecu

Ecuador identification validator.

## Documents

- Cédula (10 digits).
- RUC (13 digits).
- Passport (6-9 alphanumerics).

## Validation algorithm

Cédula: province `01-24` or `30` (registered abroad), third digit `0-5`,
modulo 10 with weights `[2,1,2,1,2,1,2,1,2]` (products above 9 have 9
subtracted). RUC: 13 digits, province `01-24`, `30` or the legacy `50`, always
ending in `001` (public entities in `0001`). Third digit `0-5` is a natural
person: a valid cédula followed by `001`. Third digit `6` is a public entity:
modulo 11 with weights `[3,2,7,6,5,4,3,2]` over the 8-digit body, tenth digit
`0`. Third digit `9` is a private company or a foreign person without a
cédula: only the structure is checked, because the SRI stopped guaranteeing
the modulo 11 digit when the sequential passes six digits and recommends
querying its catastro instead. A hyphen written right before the last digit is
tolerated (`171234567-5`).

### Editable constants

- `MAX_PROVINCE_CODE = 24` (`src/validators/id_card.rs`) — provinces are
  `01`-`24`; edit when one is created or merged.
- `ABROAD_PROVINCE_CODE = 30` (`src/validators/id_card.rs`) — series for
  Ecuadorians registered abroad, per the Registro Civil.
- `LEGACY_CODE = 50` (`src/validators/ruc.rs`) — not a province and absent
  from the SRI reglamento; it only appears in python-stdnum's prefix list.
  Kept so legacy RUCs are not rejected; remove when the SRI confirms what `50`
  identifies.

## Components

`components(value)` (and the `Validation` returned by `validate`) expose the
parts encoded in the number. `Ok(None)` for a valid passport.

- Cédula: `province` (INEC code and name; code `30` is registered abroad),
  `person_type` (third digit, `0`-`5`), `serial` (digits 4-9) and `check_digit`.
- RUC: `province`, `taxpayer` (`NaturalPerson` for `0`-`5`, `PublicEntity` for
  `6`, `PrivateCompany` for `9`). Natural persons also get the embedded cédula
  as `id_card`; public entities get their five-digit `serial`.

```rust
use idenumbra_ecu::{components, Components};

if let Ok(Some(Components::IdCard(card))) = components("1712345675") {
    assert_eq!(card.province.name, Some("Pichincha"));
    assert_eq!(card.check_digit, 5);
}
```

## References

Official sources are listed first; the `implementation` ones are the
cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — SRI, Reglamento a la Ley del RUC (Decreto 2167, updated
  2024-02-09), art. 3: 13 digits, province in the first two, `001` for every
  taxpayer, natural person = cédula + `001`:
  https://www.sri.gob.ec/o/sri-portlet-biblioteca-alfresco-internet/descargar/cc9751bf-9123-4219-bf2c-79daba57116a/Reglamento_Ley_RUC_ultmodificacion_09022024.pdf
- **Official** — OECD, *Information on Tax Identification Numbers – Ecuador*
  (structure supplied by the SRI: third digit `9` private, `6` public, tenth
  digit `0` for public entities, last three `001`):
  https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/ecuador-tin.pdf
- **Official** — Registro Civil, *Preguntas frecuentes – Cedulación* (cédula
  series: `01`-`24` in the country, `30` for Ecuadorians registered abroad):
  https://www.registrocivil.gob.ec/preguntas-frecuentes-cedulacion/
- **Official** — INEC, División Política Administrativa (province codes):
  https://www.ecuadorencifras.gob.ec/documentos/web-inec/Geografia_Estadistica/Micrositio_geoportal/index.html
- **Official (quoted)** — SRI comunicado on RUC generation (2020 oficio
  `SRI-NAC-SGD-2020-0092-O`, 2021 comunicado), reproduced in full in a Gob.EC
  issue: the modulo 11 digit is only applied while the sequential has up to six
  digits, and the SRI recommends not validating RUCs algorithmically:
  https://minka.gob.ec/mintel/ge/rutr/gobec_forms/-/issues/32
- **Implementation** — python-stdnum `ec.ruc` (where province `50` and the
  `6`/`9` fallbacks come from):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/ec/ruc.py

Neither the SRI nor the Registro Civil publishes the cédula modulo 10 or the
public-entity modulo 11 algorithm; both are reconstructed from the values the
SRI validates (ficha técnica de comprobantes electrónicos, warning 62) and from
the implementations above. Company RUCs are deliberately not check-digit
validated.

Links were opened on 2026-11-17.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["ecu"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("ECU", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `1712345675`
- RUC (natural person): `1712345675001`
- RUC (private company): `1790012344001`
- RUC (private company with a mismatched modulo 11 digit): `1790012345001`
- RUC (public company): `1760001200001`
- RUC (legacy province `50`): `5090012340001`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SRI modulo 10/11 rules.

## Notes

- Feature flag: `ecu`.
- Country code (ISO 3166-1 alpha-3): `ECU`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Private-company RUCs only get a structural check; `1790012345001` passes even
  though a modulo 11 digit would be `4`.
