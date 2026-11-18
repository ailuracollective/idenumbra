# idenumbra-cri

Costa Rica identification validator.

## Documents

- Cédula de identidad (9 digits).
- DIMEX (11-12 digits).
- Passport (alphanumeric).

## Validation algorithm

Length based: 9 digits for the cédula de identidad física, 11-12 digits for the
DIMEX and 6-12 alphanumerics for a passport. Costa Rican identity numbers carry
no public check digit, so no arithmetic check is applied.

### Editable constants

- `DIMEX_LENGTHS = [11, 12]` (`src/validators/validate.rs`) — the DIMEX is 11
  digits in the legacy format and 12 in the newer one; the tax authority
  accepts both and tells filers to left-pad the 11-digit ones with a zero.
  Drop `11` only if the legacy format is retired.

## References

- **Official** — Ministerio de Hacienda, *Anexos y Estructuras* v4.4 (cédula =
  9 digits, DIMEX = 11 or 12 digits): https://www.hacienda.go.cr/docs/ANEXOS_Y_ESTRUCTURAS_V4.4.pdf
- **Official** — Dirección General de Migración y Extranjería, DIMEX: https://migracion.go.cr/dimex/
- **Official** — Tribunal Supremo de Elecciones, cédula de identidad: https://www.tse.go.cr/cedula.html
- **Official (international)** — OECD, *Costa Rica TIN*: https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/costa-rica-tin.pdf
- **Implementation** — https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Costa_Rica)

No official source publishes a cédula or DIMEX check digit; the TSE issues and
verifies the numbers itself.

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
idenumbra = { version = "0.1", features = ["cri"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("CRI", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `123456789`
- DIMEX: `12345678901`

These values are **synthetic** and do **not** correspond to any real person. Generated locally matching the validator format rules. Source: TSE and Dirección General de Migración y Extranjería document formats.

## Notes

- Feature flag: `cri`.
- Country code (ISO 3166-1 alpha-3): `CRI`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
