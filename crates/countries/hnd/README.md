# idenumbra-hnd

Honduras identification validator.

## Documents

- DNI (13 digits).
- RTN (14 digits).
- Passport (6-12 alphanumerics).

## Validation algorithm

DNI: 13 digits, no check digit (the RNP publishes neither the digit layout nor
an algorithm; the 4+4+5 `municipio/año/correlativo` split is visible in public
data, not an official rule). RTN: 14 digits; for natural persons it is the DNI
plus one digit, per the SAR. The SAR and the SEFIN/SDE publish the length but not
the verifier algorithm, so only the shape is checked (all-zero placeholders are
rejected). Hyphens and spaces are ignored.

What is **not** validated, and why:

- The RTN verifier digit. The community weights
  `[2,7,6,5,4,3,2,7,6,5,4,3,2]` (mod 10) match only 15 of 124 supplier RTNs
  published by ONCAE in open contracting data (2014-2025) — about chance. A
  sweep of modulus 10/11 with ascending/descending weight cycles up to period 9
  never exceeded 21 of 124. There is no consistent check digit to implement.
- The DNI internal layout. Public RTNs of natural persons show plausible
  municipality and year blocks, but no RNP document defines the fields.

### Editable constants

- `DNI_LEN = 13` / `RTN_LEN = 14` (`src/validators/rtn.rs`) — document lengths.
  Edit when the RNP changes the DNI format or the SAR changes the RTN format.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — SAR, *Registro Tributario Nacional (RTN)*: "El RTN de Personas
  Naturales es su número de DNI con un dígito adicional", four taxpayer types,
  no algorithm:
  https://www.sar.gob.hn/registro-tributario-nacional-rtn
- **Official** — Secretaría de Desarrollo Económico, *RTN numérico*: numeric
  arrangement of 14 positions since 2003, based on the identity card number for
  natural persons:
  https://sde.gob.hn/registro-tributario-nacional-numerico-rtn
- **Official open data** — ONCAE, *Datos abiertos* (HonduCompras/OCDS releases;
  supplier identifiers under scheme `HN-RTN`, used here as the sample of real
  RTNs): https://datosabiertos.oncae.gob.hn/datosabiertos/data.json
- **Third party** — TIN Validate, Honduras RTN (states a verifier digit exists
  but publishes no algorithm; not confirmed by the SAR):
  https://tin-validate.com/docs/countries/hn
- **Third party** — Box4Dev RTN generator/validator (source of the community
  weights `[2,7,6,5,4,3,2,7,6,5,4,3,2]`; the SAR does not publish them and real
  RTNs do not validate with them): https://box4.dev/es/honduras/generador-rtn
- **Third party** — org-id.guide, HN-RTN (format examples only):
  https://org-id.guide/list/HN-RTN

An earlier version of this crate enforced the community weights; the two public
`org-id.guide` example RTNs fail them and so do 109 of 124 official supplier
RTNs, so the check was removed. Use the SAR "Verificación de RTN" service for
existence.

Official links checked on 2026-11-17; the third-party links are public
cross-checks, not the rule source.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["hnd"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("HND", "12345678")?;
```

## Test numbers (synthetic, not real)

- DNI: `1234567890123`
- RTN: `12345678901230` (same value plus one digit, like a natural person's RTN)

These values are **synthetic** and do **not** correspond to any real person or
company. Generated locally for the structural rules. Source: generated locally;
no check digit is computed. Property tests also generate non-zero DNIs, RTNs
with every trailing digit, and alphanumeric passports locally; none is looked
up or associated with a person.

## Notes

- Feature flag: `hnd`.
- Country code (ISO 3166-1 alpha-3): `HND`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The RTN verifier digit is not checked, by design: `validate("...31")` and
  `validate("...30")` are equally accepted. Do not use this crate as the sole
  check for tax IDs.
