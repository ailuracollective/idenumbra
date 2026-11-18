# idenumbra-ury

Uruguay identification validator.

## Documents

- CI (7-digit body plus check digit; the oldest numbers have a 6-digit body).
- RUT (12 digits, modulo 11).
- Passport (6-12 alphanumerics).

## Validation algorithm

CI: the identification number is a natural sequence starting at 1 followed by a
module-10 check digit (Decreto 501/1978, art. 2). The decree fixes only the
modulus, not the weights or a width; the body is left-padded to 7 digits and
multiplied by `[2,9,8,7,6,3,4]`, with the check digit
`(10 - sum mod 10) mod 10`, as the public implementations document. A 6-digit
body is therefore the same number with a leading zero, and both spellings
validate.

RUT: the first two digits must be `01-22`, positions 3-8 must not be all zero,
positions 9-11 must be `001`, and the check digit uses weights
`[4,3,2,9,8,7,6,5,4,3,2]` with `11 - remainder`; remainder 0 gives `0` and
remainder 1 would give `10`, which no RUT can carry: the public implementations
discard such a sequence instead of producing a digit, so those eleven digits
are rejected as `UnrepresentableCheckDigit`. Dots, hyphens and spaces are
ignored.

### Editable constants

- `MAX_OFFICE_CODE = 22` (`src/validators/rut.rs`) — the first two RUT digits
  are the DGI operating unit where the taxpayer registered, not the department
  (the DGI now assigns `22` regardless of department). `01-21` belong to
  taxpayers registered before October 2024 and keep their numbers; `22` is the
  only prefix assigned since the week of 21 October 2024.
- `RUT_OFFICE_GROUP = b"001"` (`src/validators/rut.rs`) — middle group, `001`
  in every public sample and in both reference implementations. Some third-party
  pages claim `002+` for branches; no DGI document says so, so only `001` is
  accepted.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — Decreto 501/1978, art. 2: the identification number is a
  sequence from 1 plus the check digit "resultante de la aplicación del módulo
  diez": https://www.impo.com.uy/bases/decretos/501-1978 — checked 2026-11-17.
- **Official** — MEF, response to access-to-information request 2024-5-1-0002222
  (RUT = 2-digit prefix + 6 sequential + 3-digit suffix + check digit; the
  values and the check-digit algorithm are classified reserved by Resolución
  836/2024 of 16 April 2024):
  https://www.gub.uy/ministerio-economia-finanzas/institucional/informacion-gestion/informes-solicitudes-acceso-informacion-publica/respuesta-100
  — checked 2026-11-17.
- **Official (secondary)** — UAIP, Resolución 145/024 of 21/08/2024, which
  validates that same reservation and names Resolución 836/2024:
  https://www.gub.uy/unidad-acceso-informacion-publica/institucional/normativa/resolucion-n-145024-sobre-reserva-informacion
  — checked 2026-11-17.
- **Official** — DGI, *Nueva numeración del RUT* (from the week of 21 October
  2024 the first two digits of every new RUT are `22`, regardless of the
  department; existing taxpayers keep theirs):
  https://www.gub.uy/direccion-general-impositiva/comunicacion/noticias/nueva-numeracion-del-rut
  — checked 2026-11-17.
- **Implementation** — python-stdnum `uy.rut` (12 digits, prefix `01`-`22`,
  `001` group, weights `[4,3,2,9,8,7,6,5,4,3,2]`):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/uy/rut.py
- **Implementation** — ciuy, which documents the CI digit as the dot product
  with `<8,1,2,3,4,7,6>` modulo 10 (for the 7-digit body) and `<1,2,3,4,7,6>`
  (for the 6-digit one); each pair of weights adds up to 10 with the ones used
  here, and `(10 - sum mod 10) mod 10` is `-sum mod 10`, so the two formulations
  agree: https://ciuy.readthedocs.io/es/latest/about.html
- **Implementation** — https://github.com/picandocodigo/ci_uy (CI weights
  `2,9,8,7,6,3,4`, 6 or 7 digit body).

The DGI classifies the RUT check-digit algorithm as reserved information, so the
weights above come from public implementations, not from a norm. No agency
publishes the CI weights either; Decreto 501/1978 only fixes the modulus.

Official links checked on 2026-11-17; the implementation links are public
cross-checks, not dated.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["ury"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("URY", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `1.234.567-2` (`12345672`)
- CI with a 6-digit body: `123.456-1` (`1234561`, body padded to `0123456`)
- RUT: `21-999983-001-9` (`219999830019`)
- RUT with the 2024 prefix: `22-999983-001-6` (`229999830016`)

These values are **synthetic** and do **not** correspond to any real person or
company. Generated locally with this crate's validation algorithm.

## Notes

- Feature flag: `ury`.
- Country code (ISO 3166-1 alpha-3): `URY`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
