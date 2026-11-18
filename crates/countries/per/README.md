# idenumbra-per

Peru identification validator.

## Documents

- DNI (8 digits).
- RUC (11 digits, modulo 11).
- Carné de extranjería / passport (6-12 alphanumerics).

## Validation algorithm

DNI: 8 digits, no check digit (RENIEC publishes none). RUC: 11 digits starting
with `10`, `15`, `17` or `20`; weights `[5,4,3,2,7,6,5,4,3,2]` over the first
10 digits; check digit `(11 - sum mod 11) mod 10` (remainder 0 gives 1,
remainder 1 gives 0). That is the computation SUNAT's own form validator uses
(`validRUC` in its `normalization.js`).

### Editable constants

- `RUC_PREFIXES = [b"10", b"15", b"17", b"20"]` (`src/validators/ruc.rs`) —
  issuer prefixes: `10` natural person with DNI, `15` foreign natural person and
  `20` legal person (national or foreign) are described by SUNAT; `17` is only
  documented by implementations.

## References

Official sources are listed first; the `implementation` ones are the
cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — SUNAT, *Inscríbete en el RUC* (11 digits; national natural
  person `10` + DNI + check digit, foreign natural person `15` + random
  number, legal person `20` + random number):
  https://centrovirtual.sunat.gob.pe/tramites/inscribete-ruc
- **Official** — SUNAT, RUC legislation:
  https://www.sunat.gob.pe/legislacion/ruc/
- **Official** — SUNAT, inscription in the RUC (natural person, documents
  required): https://www.gob.pe/284-sacar-ruc-persona-natural
- **Official** — SUNAT, *Guía Tributaria y Aduanera II* (NAF), section 1
  "Inscripción al RUC": only `10`, `15` and `20` are described:
  https://cultura.sunat.gob.pe/sites/default/files/Guia-Tributaria-Aduanera_NAF.pdf
- **Official** — SUNAT, online form validator (`validRUC` in `normalization.js`):
  the same weights `5,4,3,2,7,6,5,4,3,2` and the mapping remainder 0 → 1,
  remainder 1 → 0:
  https://www.sunat.gob.pe/a/js/servicio/registro/comppago/see/rhe/js-bin/normalization.js
- **Official** — RENIEC, DNI questions (8 digits):
  https://identidad.reniec.gob.pe/preguntas-frecuentes-dni
- **Implementation** — python-stdnum `pe.ruc` (prefix table with `17` and the
  modulo 11 weights; SUNAT publishes neither):
  https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/pe/ruc.py
- **Implementation** — https://es.wikipedia.org/wiki/Documento_Nacional_de_Identidad_(Per%C3%BA)

SUNAT publishes no prefix table and no check-digit algorithm; it only names the
eleventh digit "dígito verificador", and its own form code computes it with the
weights above. `17` is accepted on implementation evidence only, and `16` (seen
in a few ports and blogs, not in python-stdnum) is not accepted. Links checked on 2026-11-17.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1.0-alpha.1", features = ["per"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("PER", "12345678")?;
```

## Test numbers (synthetic, not real)

- DNI: `12345678`
- RUC (legal person): `20123456786`
- RUC (foreign natural person): `15123456782`
- RUC (prefix `17`): `17123456785`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the SUNAT modulo 11 rules.

## Notes

- Feature flag: `per`.
- Country code (ISO 3166-1 alpha-3): `PER`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
