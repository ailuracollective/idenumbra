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
agency (Policía Nacional, Departamento de Identificaciones) publishes none. Its
own history states the CI number "es de orden correlativo, a partir del Nº 1 y
actualmente el número de emisión es el 8.198.325, de fecha 27 de marzo de 2017":
a plain counter, no check character, and already 7 digits in 2017. The 6-8 range
is therefore observed practice, not a rule, and an 8-digit CI (reachable once
the counter passes 9.999.999) is only accepted through the explicit `ci` kind;
by inference that length is read as a RUC.

RUC: modulo 11 with weights 2..11 cycling from the right; the check digit is
`11 - remainder`, where remainders 0 and 1 give `0` (DNIT rule). The body is 3-8
characters, so the full number has 4-9 digits. Dots, hyphens and spaces are
ignored.

SIFEN Nota Técnica N° 14 declares `dRucRec` as alphanumeric (`A`, 3-8
characters), because old RUCs may carry a letter (`A`-`D` before the check
digit); the DNIT algorithm replaces it with its ASCII code. That form is not
supported here: only digits are accepted. This is a documented limitation, not
an official restriction.

### Editable constants

None: this validator has no catalogue-derived constants; every check is algorithmic or structural.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — DNIT, *Dígito Verificador*: reference implementation of the RUC
  check digit (weights 2..11 from the right, `11 - remainder`, remainders 0 and 1
  give 0):
  https://www.dnit.gov.py/documents/20123/224893/D%C3%ADgito+Verificador.pdf/fb9f86c8-245d-9dad-2dc1-ac3b3dc307a7
  — checked 2026-11-17.
- **Official** — DNIT, SIFEN Nota Técnica N° 14 (20/03/2023): `dRucRec` is type
  `A`, 3-8 characters, and `dDVRec` is "Según algoritmo módulo 11"; rejection
  4463 is "El Dígito Verificador ingresado no corresponde al módulo 11 del RUC":
  https://www.dnit.gov.py/documents/20123/420595/NT_E_KUATIA_014_MT_V150X.pdf/dbbb0294-8678-357a-1657-bcd0318077f9
  — checked 2026-11-17.
- **Official** — Policía Nacional, Departamento de Identificaciones, *Quiénes
  somos*: the CI number is a correlative from Nº 1, at 8.198.325 on 27/03/2017,
  and the page describes only the separate inscription number, so no length or
  check digit is published for the CI:
  https://www.policianacional.gov.py/identificaciones/quienes-somos/ —
  checked 2026-11-17.
- **Official** — Policía Nacional, *Cédula de Identidad* (the open
  identification service; the number is on the card):
  https://www.policianacional.gov.py/identificaciones/cedula-de-identidad —
  checked 2026-11-17.
- **Implementation** — python-stdnum `py.ruc` (up to 9 digits, modulo 11 from
  the right, `0`/`1` remainder mapped to 0):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/py/ruc.py

The RUC check digit is published; the CI length is not, so the 6-8 digit range is
observed practice rather than a norm.

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
idenumbra = { version = "0.1.0-alpha.1", features = ["pry"] }
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
