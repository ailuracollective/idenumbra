# idenumbra-pry

Paraguay identification validator.

## Documents

- CI (cédula de identidad, 6-8 digits, no check digit).
- RUC (3-8 digit body plus check digit, modulo 11).
- Passport (6-12 alphanumerics).

Without a kind, a 6-7 digit number is treated as a CI and an 8-9 digit one as a
RUC. The shapes overlap: an 8-digit RUC (7-digit CI plus check digit) and an
8-digit CI are the same length, and a 9-digit RUC carries an 8-digit CI. Pass
the kind (`validate_with(.., "ci" | "ruc")`) when the document type is known.

## Validation algorithm

CI: 6-8 digits, no check digit; only the shape is checked, because the issuing
agency (Policía Nacional, Departamento de Identificaciones) publishes none. An
8-digit CI is only accepted through the explicit `ci` kind; by inference that
length is read as a RUC.

RUC: modulo 11 with weights 2..11 cycling from the right; the check digit is
`11 - remainder`, where remainders 0 and 1 give `0` (DNIT rule). The body is 3-8
characters, so the full number has 4-9 digits. Dots, hyphens and spaces are
ignored.

Old RUCs may end in a letter (`A`-`D` before the check digit); the DNIT
algorithm replaces it with its ASCII code. That form is not supported here: only
digits are accepted.

## References

- **Official** — DNIT, *Dígito Verificador*: reference implementation of the RUC
  check digit (weights 2..11 from the right, `11 - remainder`, remainders 0 and 1
  give 0):
  https://www.dnit.gov.py/documents/20123/224893/D%C3%ADgito+Verificador.pdf/fb9f86c8-245d-9dad-2dc1-ac3b3dc307a7
  — checked 2026-11-17.
- **Official** — DNIT, SIFEN Nota Técnica N° 14 (20/03/2023): `dRucRec` is 3-8
  characters and `dDVRec` is "según algoritmo módulo 11"; rejection 4463 is a
  check digit that does not match the RUC's modulo 11:
  https://www.dnit.gov.py/documents/20123/420595/NT_E_KUATIA_014_MT_V150X.pdf/dbbb0294-8678-357a-1657-bcd0318077f9
  — checked 2026-11-17.
- **Official** — Policía Nacional, Departamento de Identificaciones (issuer of
  the CI; the number is on the card but no length or check digit is published):
  https://www.policianacional.gov.py/identificaciones/cedula-de-identidad —
  checked 2026-11-17.
- **Implementation** — python-stdnum `py.ruc` (up to 9 digits, modulo 11 from
  the right, `0`/`1` remainder mapped to 0):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/py/ruc.py

The RUC check digit is published; the CI length is not, so the 6-8 digit range is
observed practice rather than a norm.

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
idenumbra = { version = "0.1", features = ["pry"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("PRY", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `1234567`
- RUC: `80001234-8` (`800012348`)
- RUC, shortest body: `123-6` (`1236`)

These values are **synthetic** and do **not** correspond to any real person or
company. Generated locally with this crate's validation algorithm.

## Notes

- Feature flag: `pry`.
- Country code (ISO 3166-1 alpha-3): `PRY`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Without a kind, 6-7 digit numbers are treated as CI and 8-9 digit numbers as
  RUC; use `validate_with("ci" | "ruc")` to disambiguate.
