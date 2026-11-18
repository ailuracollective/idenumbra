# idenumbra-hnd

Honduras identification validator.

## Documents

- DNI (13 digits).
- RTN (14 digits).
- Passport (6-12 alphanumerics).

## Validation algorithm

DNI: 13 digits (`municipio(4) + año(4) + correlativo(5)`), no check digit. RTN:
14 digits; for natural persons it is the DNI plus one digit. The SAR publishes
the length but not the verifier algorithm, so only the shape is checked
(all-zero placeholders are rejected). Hyphens and spaces are ignored.

### Editable constants

- `DNI_LEN = 13` / `RTN_LEN = 14` (`src/validators/rtn.rs`) — document
  lengths. Edit when the RNP/SAR changes them.

## References

- **Official** — SAR, *Registro Tributario Nacional (RTN)* (RTN of a natural
  person is the DNI plus one digit):
  https://www.sar.gob.hn/registro-tributario-nacional-rtn
- **Official** — Secretaría de Desarrollo Económico, *RTN numérico* (14
  positions since 2003):
  https://sde.gob.hn/registro-tributario-nacional-numerico-rtn
- **Official** — TIN Validate, Honduras RTN (documents the 14-digit layout and
  that the last digit is a verifier, without the algorithm):
  https://tin-validate.com/docs/countries/hn
- **Implementation** — Box4Dev RTN generator/validator (community weights
  `[2,7,6,5,4,3,2,7,6,5,4,3,2]`; the SAR does not publish them and public
  example RTNs do not validate with them):
  https://box4.dev/es/honduras/generador-rtn

No SAR document publishes the RTN check-digit algorithm. An earlier version of
this crate enforced the community weights; the two public `org-id.guide`
example RTNs fail them, so the check was removed. Use the SAR "Verificación de
RTN" service for existence.

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
idenumbra = { version = "0.1", features = ["hnd"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("HND", "12345678")?;
```

## Test numbers (synthetic, not real)

- DNI: `1234567890123`
- RTN: `12345678901230`

These values are **synthetic** and do **not** correspond to any real person or
company. Generated locally for the structural rules. Source: generated locally;
no check digit is computed.

## Notes

- Feature flag: `hnd`.
- Country code (ISO 3166-1 alpha-3): `HND`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The RTN verifier digit is not checked, by design: `validate("...31")` and
  `validate("...30")` are equally accepted. Do not use this crate as the sole
  check for tax IDs.
