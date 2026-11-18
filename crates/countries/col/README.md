# idenumbra-col

Colombia identification validator.

## Documents

- CC (cédula de ciudadanía, 6-10 digits).
- NIT (1-11-digit body plus check digit, modulo 11).
- Passport (alphanumeric).

## Validation algorithm

CC: 6-10 digits, no check digit. NIT: weights from the right
`[3,7,13,17,19,23,29,37,41,43,47]` (the leftmost body digit is multiplied by
41 for a 9-digit body, per DIAN Orden Administrativa 4/1989, annex 3); check
digit is the remainder when it is 0 or 1, otherwise `11 - remainder`. Bodies of
1-9 digits are what the DIAN publishes; 10 and 11 digits are natural persons
whose NIT is their cédula or civil registry number, and the prime series is
continued (`43`, `47`) as every implementation does. Dots, hyphens and spaces
are ignored.

Bodies of 12 or more digits are rejected: a cédula has at most 10 digits and the
civil registry number 11, so 12 digits with the check digit is the longest NIT
any DIAN document describes. The 15-prime table some implementations carry
(padding the number to 15 digits) is not in the DIAN's annex and is not used
here.

The cédula carries no check digit: the Registraduría publishes none for the
NUIP. The old 11-digit NIP of minors did end in a control digit, but that is the
tarjeta de identidad number, not the cédula.

## References

Official sources are listed first; the `implementation` ones are the
cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — DIAN, Orden Administrativa 4 de 1989, annex 3 (weights
  `41,37,29,23,19,17,13,7,3` right to left, remainder 0/1 kept, otherwise
  `11 - remainder`), on the DIAN's legal compilation:
  https://normograma.dian.gov.co/dian/compilacion/docs/orden_administrativa_dian_0004_1989.htm
- **Official** — DIAN, Concepto 78424 de 2002: a natural person's NIT is their
  cédula number, or an 11-digit civil registry number for minors, always plus
  the DIAN check digit:
  https://normograma.dian.gov.co/dian/compilacion/docs/concepto_tributario_dian_0078424_2002.htm
- **Official** — OECD, *Information on Tax Identification Numbers – Colombia*
  (companies: 9 digits plus check digit; natural persons: 1 to 13 digits plus
  check digit):
  https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/colombia-tin.pdf
- **Official** — Registraduría Nacional, Resolución 3571 de 2003 (the NUIP, i.e.
  the cédula number, is 10 numeric digits; older cédulas keep fewer):
  https://cancilleria.gov.co/sites/default/files/Normograma/docs/pdf/resolucion_registraduria_3571_2003.pdf
- **Official** — Registraduría Nacional, *Ahora la cédula es de 10 dígitos*
  (cédulas issued with up to 8 digits keep their number):
  https://www.registraduria.gov.co/Ahora-la-cedula-es-de-10-digitos.html
- **Implementation** — python-stdnum `co.nit` (the extended prime series
  `43,47,53,59,67,71` used for bodies longer than 9 digits):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/co/nit.py

The check digit is the DIAN's; the weights beyond the ninth position are not in
annex 3 and are taken from the implementations above. Links were opened on
2026-11-17.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["col"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("COL", "12345678")?;
```

## Test numbers (synthetic, not real)

- CC: `12345678`
- NIT: `900.657.800-0` (`9006578000`), validated with `validate_with(.., "nit")`
- NIT: `987.654.321-7`
- NIT (8-digit body, short natural person): `12345678-8`
- NIT (10-digit body, natural person with a 10-digit cédula): `1098765432-9`
- NIT (11-digit body, civil registry number): `10987654321-4`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the validator algorithm; DIAN prime weights.

## Notes

- Feature flag: `col`.
- Country code (ISO 3166-1 alpha-3): `COL`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Cédulas and NITs overlap: 10-digit numbers without a kind are validated as cédulas, so use `validate_with("nit")` to check a NIT check digit.
- Bodies longer than 11 digits are rejected; the DIAN publishes weights for
  nine and the two extra positions cover cédulas and civil registry numbers.
