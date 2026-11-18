# idenumbra-nic

Nicaragua identification validator.

## Documents

- Cédula de identidad (13 digits plus a check letter).

## Validation algorithm

Layout: `municipio(3) + birth date DDMMYY(6) + serial(4) + letter`. The law (Ley
152, art. 31) defines the 13 digits as the municipality code, the birth date and
four progressive digits; the final check letter is not mentioned there. As in
the independent validators, the letter is the 13-digit number modulo 23, indexed
in `ABCDEFGHJKLMNPQRSTUVWXY` (23 letters, no `I`, `O` or `Z`), and the encoded
birth date must be a real calendar date. Hyphens and spaces are ignored. Passports
are validated only when the `passport` kind is requested.

## References

Official:

- Ley 152, *Ley de Identificación Ciudadana*, texto consolidado, art. 31 (13
  digits: municipality 3, birth date 6, progressive 4), La Gaceta No. 9 of
  2024-01-18:
  http://legislacion.asamblea.gob.ni/normaweb.nsf/b92aaea87dac762406257265005d21f7/64f20479f029610506258ab60059454d?OpenDocument —
  checked 2026-11-17
- CSE, *Cedulación* (the CSE issues the cédula under Ley 152):
  https://www.cse.gob.ni/cedulacion — checked 2026-11-17
- CSE, new cédula format in force since 2026-02-21 (redesign only; the number
  stays the same): https://www.hacienda.gob.ni/nuevo-formato-de-cedula-de-identificacion-ciudadana —
  checked 2026-11-17
- Ley 152 is the text in force again: Ley 1241 (March 2025) was repealed in June
  2025 by Ley 1254, *Ley de Restablecimiento de la Ley 152*:
  https://www.tn8.tv/nacionales/derogada-la-ley-1241-nicaragua-vuelve-a-su-modelo-de-identificacion-ciudadana —
  checked 2026-11-17

Implementations (cross-checks, not authority):

- https://gist.github.com/ernestoruiz89/9d92c92343ebd374e13ce1a2b6b2ac4d (SQL:
  `number mod 23` indexed in `ABCDEFGHJKLMNPQRSTUVWXY`)
- https://www.roderickzapata.com/herramientas/verificar-cedula-nicaragua/
  (14 characters, `DDMMAA`, the same 23 letters, `I`/`O`/`Z` excluded)
- https://appnica.com/api_verificar_cedula (example `0010101010011` -> `H`,
  reproduced locally)
- https://github.com/momotombodevs/stdnum-centroamerica (`ni.cedula`: four
  implementations and one MRZ sample agree; its research notes are dated
  2026-10-19)

The CSE does not publish the modulo-23 letter rule; it is implemented by
independent validators and matches the CSE's own check when issuing.

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
Idenumbra::infer("NIC", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `001-010190-0001N`
- Cédula born on 29 February 2024: `0012902400001C`

These values are **synthetic** and do **not** correspond to any real person.
They were generated locally with this crate's validation algorithm.

## Notes

- Feature flag: `nic`.
- Country code (ISO 3166-1 alpha-3): `NIC`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Municipality code existence is not checked: the CSE does not publish the
  three-digit list, the third-party lists disagree (some omit Mulukukú) and the
  code `888` is reported for people born abroad by a single source. The check
  letter and the birth date are validated.
- `validate` handles only the cédula; use `validate_with(.., "passport")` for
  passports.
- What is not validated: whether the cédula exists in the CSE registry, and the
  department/municipality the first three digits refer to.
