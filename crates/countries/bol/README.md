# idenumbra-bol

Bolivia identification validator.

## Documents

- CI (identity card, 5-8 digits plus an optional 1-2 character complement).
- NIT (tax number, 7-12 digits).

## Validation algorithm

CI: 5-8 digits; the complement (when present) is 1-2 ASCII alphanumerics,
optionally after `-` or a space. NIT: 7-12 digits. Neither document has a
published check digit: the NIT ends in three control digits assigned by the
SIN, and the SIAT "Módulo 11" page documents the CUF (invoice code), not the
NIT. All-zero placeholders are rejected; everything else is structure only.

### Editable constants

- `NIT_MIN_LEN = 7` / `NIT_MAX_LEN = 12` (`src/validators/nit.rs`) — NIT length
  range observed in the SIN registry. Edit when the SIN changes the numbering.

## References

- **Official** — SIN/SIAT, *Generación del NIT* (natural persons use their CI;
  legal persons get an assigned number):
  https://siatinfo.impuestos.gob.bo/index.php/requisitos-para-la-inscripcion/conceptos-generales/generacion-del-nit
- **Official** — SIN/SIAT, *Algoritmo Módulo 11* (explicitly the CUF, not the
  NIT): https://siatinfo.impuestos.gob.bo/index.php/facturacion-en-linea/algoritmos-utilizados/algoritmo-modulo-11
- **Implementation** — python-stdnum, *Add support for Bolivia TIN* (NIT = CI +
  three control digits, up to 12 digits; no published algorithm):
  https://github.com/arthurdejong/python-stdnum/issues/134
- **Implementation** — org-id.guide, BO-NIT (format examples):
  https://org-id.guide/list/BO-NIT

The SIN publishes no NIT check-digit algorithm and no CI check digit. An
earlier version of this crate applied the SIAT CUF modulo 11 to the NIT; the
public NIT samples in the python-stdnum issue do not validate with it, so the
check was removed. Use the SIN "Verifica NIT" service for existence.

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
idenumbra = { version = "0.1", features = ["bol"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("BOL", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `1234567`
- CI with complement: `1234567-1A`
- NIT: `123456789`

These values are **synthetic** and do **not** correspond to any real person or
company. Generated locally for the structural rules. Source: generated locally;
no check digit is computed.

## Notes

- Feature flag: `bol`.
- Country code (ISO 3166-1 alpha-3): `BOL`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- A 7-8 digit number is treated as a CI; use `validate_with("nit")` to check an
  shorter NIT.
