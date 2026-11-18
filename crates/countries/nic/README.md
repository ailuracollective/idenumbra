# idenumbra-nic

Nicaragua identification validator.

## Documents

- Cédula de identidad: 13 digits plus a check letter (14 characters).
- Pasaporte: 6–12 alphanumeric characters, structure only.
- Kinds: `id_card`, `passport`. `validate` infers the kind from the shape;
  `validate_with(value, kind)` requires it.

## Validation algorithm

Cédula. Layout: `municipio(3) + birth date DDMMYY(6) + serial(4) + letter`. The
law (Ley 152, art. 31) defines the 13 digits as the municipality code, the birth
date and four progressive digits; the final check letter is not mentioned there
and is not part of the legal number. The letter rule is **community-derived**:
the CSE publishes no check-character algorithm, so the rule below is taken from
independent validators and from the arithmetic they agree on, not from an
official specification. As in those validators, the letter is the 13-digit
number modulo 23, indexed in `ABCDEFGHJKLMNPQRSTUVWXY` (23 letters, no `I`, `O`
or `Z`), and the encoded birth date must be a real calendar date. Years are read
as `20YY`, which only misjudges 29 February 1900. Hyphens and spaces are
ignored.

Pasaporte. Only the shape is checked: 6–12 ASCII alphanumerics. No issuing
agency publishes a passport-number algorithm, so nothing else is verifiable.

## References

Official:

- Ley 152, *Ley de Identificación Ciudadana*, texto consolidado, art. 31 (13
  digits: municipality 3, birth date 6, progressive 4), La Gaceta No. 9 of
  2024-01-18:
  http://legislacion.asamblea.gob.ni/normaweb.nsf/b92aaea87dac762406257265005d21f7/64f20479f029610506258ab60059454d?OpenDocument —
  checked 2026-11-17 (the site rejects automated requests; the text was read
  through the search index)
- CSE, *Cedulación* (the CSE issues the cédula under Ley 152):
  https://www.cse.gob.ni/cedulacion — checked 2026-11-17
- CSE, new cédula format in force since 2026-02-21: redesign only (larger photo,
  flag, QR, MRZ; the issue date and the director's signature removed):
  https://www.hacienda.gob.ni/nuevo-formato-de-cedula-de-identificacion-ciudadana —
  checked 2026-11-17
- CSE director general of Cedulación, on the new format: “Es un número perpetuo,
  lo tenemos desde que nacemos hasta que morimos”, so the number does not change:
  https://www.el19digital.com/articulos/ver/173996-nuevo-formato-de-cedula-presenta-mayores-estandares-de-seguridad —
  checked 2026-11-17
- Ley 152 is the text in force again: Ley 1241 (March 2025) was repealed in June
  2025 by Ley 1254, *Ley de Restablecimiento de la Ley 152*:
  https://www.tn8.tv/nacionales/derogada-la-ley-1241-nicaragua-vuelve-a-su-modelo-de-identificacion-ciudadana —
  checked 2026-11-17

Municipality codes and the check letter (no official source):

- INIDE, *Cifras Municipales*, the official division in 153 municipalities. Its
  codes are `department(2) + municipality(2)` and do not map to the three digits
  of the cédula: https://www.inide.gob.ni/docu/censos2005/CifrasMun/tablas_cifras.htm —
  checked 2026-11-17
- CSE, *Consulta Código de Persona*, the only CSE page that lists municipalities:
  names only, no codes and no 888:
  https://cse.gob.ni/es/Registrocivil/ConsultaCodigoPersona — checked 2026-11-17

Implementations (cross-checks, not authority):

- https://gist.github.com/ernestoruiz89/9d92c92343ebd374e13ce1a2b6b2ac4d (SQL:
  `number mod 23` indexed in `ABCDEFGHJKLMNPQRSTUVWXY`)
- https://www.roderickzapata.com/herramientas/verificar-cedula-nicaragua/
  (14 characters, `DDMMAA`, the same 23 letters, `I`/`O`/`Z` excluded)
- https://appnica.com/api_verificar_cedula (example `0010101010011` -> `H`,
  reproduced locally: residue 7 in the alphabet above)
- https://github.com/momotombodevs/stdnum-centroamerica (`ni.cedula`: four
  implementations and one MRZ sample agree; its research notes are dated
  2026-10-19)
- Two municipality lists that contradict each other:
  https://www.roderickzapata.com/herramientas/codigo-cedula-municipio-nicaragua/
  (2026-10-22, 153 codes plus 888) and
  https://es.scribd.com/document/748003621/CODIGO-MUNICIPALES-CEDULA-DE-IDENTIDAD-docx
- https://github.com/berroteran/valida-Cedula-nicaragua (JS: `mod 23` over the
  13 digits with the same 23-letter alphabet, no municipality check)
- https://github.com/bnorvinjosue/cedula-nicaragua-validator (JS: same letter
  rule and a date check that accepts 29 February in every year; no municipality
  check)
- https://github.com/johs7/validador-cedula-nicaragua (TS: validates the
  municipality against a 154-code map without `888` and reads `YY < 30` as
  20YY, but **does not check the letter at all** — the opposite trade-off; not
  followed)

The CSE publishes neither the modulo-23 letter rule nor the municipality codes.
The letter is implemented by independent validators and matches the CSE's own
check when issuing; the municipality table below is deliberately absent.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["nic"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("NIC", "001-010190-0001N")?;
```

## Test numbers (synthetic, not real)

- Cédula: `001-010190-0001N`
- Cédula born on 29 February 2024: `0012902400001C`
- Cédula with the born-abroad code: `8880101900001P`
- Passport: `A1234567`

These values are **synthetic** and do **not** correspond to any real person.
They were generated locally with this crate's validation algorithm.

## Notes

- Feature flag: `nic`.
- Country code (ISO 3166-1 alpha-3): `NIC`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Municipality code existence is not checked. The CSE never publishes the
  three-digit list, and the two circulating lists disagree: one has no Mulukukú
  (created in 2005), the other gives it `629` and moves San José de Bocay from
  `245` to `248`; the same list labels `041` as “Jinotega” (it is Jinotepe) and
  writes “Villa Nueva” for Villanueva. Code `888` for people born abroad comes
  from a single third-party source with no norm behind it. Validating against
  any of them would reject real cédulas, so only the check letter and the birth
  date are checked.
- `validate` treats 13–14 characters as a cédula and a 6–12 character value
  containing a letter as a passport; a shorter digit-only run is reported as a
  malformed cédula instead of being accepted as a passport. Use
  `validate_with(.., "passport")` when the document type is known.
- What is not validated: whether the cédula exists in the CSE registry, the
  municipality the first three digits refer to, and the passport number itself.
