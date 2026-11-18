# idenumbra-ury

Uruguay identification validator.

## Documents

- CI (6-7 digit body plus check digit).
- RUT (12 digits, modulo 11).
- Passport (6-12 alphanumerics).

## Validation algorithm

CI: the body is left-padded to 7 digits and multiplied by
`[2,9,8,7,6,3,4]`; the check digit is `(10 - sum mod 10) mod 10`. RUT: the
first two digits must be `01-22`, positions 3-8 must not be all zero,
positions 9-11 must be `001`, and the check digit uses weights
`[4,3,2,9,8,7,6,5,4,3,2]` with `11 - remainder` (`11 -> 0`; a check digit of 10
is not representable). Dots, hyphens and spaces are ignored.

### Editable constants

- `MAX_OFFICE_CODE = 22` (`src/validators/rut.rs`) — first two RUT digits are
  the DGI operating unit where the taxpayer registered. `01-21` are historical
  and `22` is the only one assigned since October 2024.
- `RUT_OFFICE_GROUP = b"001"` (`src/validators/rut.rs`) — middle group, a
  constant per the DGI.

## References

- **Official** — DGI, *Nueva numeración del RUT* (since October 2024 every new
  RUT starts with `22`): https://www.gub.uy/direccion-general-impositiva/comunicacion/noticias/nueva-numeracion-del-rut
- **Official** — MEF, response to access-to-information request 2024-5-1-0002222
  (RUT = 2-digit operating unit + 6 sequential + 3 fixed + check digit; the
  check-digit algorithm itself is withheld by Resolución 836/2024):
  https://www.gub.uy/ministerio-economia-finanzas/institucional/informacion-gestion/informes-solicitudes-acceso-informacion-publica/respuesta-100
- **Official** — AGESIC, *Framework para la Gestión de la Calidad de Datos*
  (cédula = 7 digits + check digit):
  https://www.gub.uy/agencia-gobierno-electronico-sociedad-informacion-conocimiento/sites/agencia-gobierno-electronico-sociedad-informacion-conocimiento/files/documentos/publicaciones/Framework%20para%20la%20Gestio%CC%81n%20de%20la%20Calidad%20de%20Datos%20v1.4.pdf
- **Implementation** — https://ciuy.readthedocs.io/es/latest/about.html
- **Implementation** — https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/uy/rut.py

The DGI treats the RUT and CI check-digit algorithms as reserved information,
so the weights below come from public implementations, not from a norm.

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
idenumbra = { version = "0.1", features = ["ury"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("URY", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `1.234.567-2` (`12345672`)
- RUT: `21-999983-001-9` (`219999830019`)

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the DGI modulo 10/11 rules.

## Notes

- Feature flag: `ury`.
- Country code (ISO 3166-1 alpha-3): `URY`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
