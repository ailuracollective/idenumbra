# idenumbra-ven

Venezuela identification validator.

## Documents

- CI (6-8 digits).
- RIF (letter `V`/`E`/`J`/`P`/`G`/`C` plus 8 digits and a check digit).
- Passport (6-12 alphanumerics).

## Validation algorithm

CI: 6-8 digits, no check digit (the cédula number is not self-checking); all-zero
placeholders are rejected. RIF: the letter contributes
`V=4, E=8, J=12, P=16, G=20, C=12`; the 8 body digits use weights
`[3,2,7,6,5,4,3,2]`; the check digit is `11 - (sum mod 11)` where 10 and 11 are
written as `0` (remainder 0 and 1 both give `0`). Hyphens and spaces are ignored;
lowercase letters are accepted.

Inference: 10 characters with a known letter and 9 digits are a RIF; 6-8 digits
are a CI; anything else alphanumeric is a passport (6-12). Use `validate_with`
when the kind is known — a passport number of 8 digits is read as a CI.

### Editable constants

- `RIF_PREFIXES` (`src/validators/validate.rs`) — letter → checksum base:
  `V` venezolano, `E` extranjero, `J` jurídico, `P` pasaporte, `G` gobierno,
  `C` comuna/consejo comunal. `C` is an alias of `J` (same base `12`): SENIAT
  introduced it in 2015 by migrating the comunas that already had a `J` RIF, so
  the eight digits and the check digit stayed, and the base cannot differ from
  `J`. These six are the letters in circulation; edit when SENIAT issues a new
  one.

## References

- **Official** — SENIAT: http://www.seniat.gob.ve/ — checked 2026-11-17 (the site
  answers `403` to automated clients; open it in a browser). SENIAT publishes
  neither the letter table nor the check-digit formula.
- **Implementation** — python-stdnum `ve.rif` (weights `[3,2,7,6,5,4,3,2]`,
  `00987654321[sum % 11]`, which is `11 - rest` with 10/11 written as `0`; it
  knows `V/E/J/P/G` but not `C`):
  https://raw.githubusercontent.com/arthurdejong/python-stdnum/master/stdnum/ve/rif.py
  — checked 2026-11-17
- **Cross-check** — the `C` letter for comunas with base `12`, plus a worked
  example (`J-30468971-3`):
  https://es.wikipedia.org/wiki/Registro_%C3%9Anico_de_Informaci%C3%B3n_Fiscal
  — checked 2026-11-17
- **Cross-check** — CDQ `VE_RIF` data-quality specification: letter weights
  `V=1, E=2, J=3, C=3, P=4, G=5`, multiplied by 4 (so `C` = `J` = 12):
  https://meta.cdq.com/DataModel:CDQ/Business_Partner/Identifier/VE_RIF
  — checked 2026-11-17
- **Secondary** — press report of the 2015 launch: the comunas and consejos
  comunales already registered were migrated from the letter `J` to `C`, which
  is why both keep the same base:
  https://elestimulo.com/venezuela/2015-08-15/seniat-imprime-primer-rif-para-comunas-con-la-letra-c/
  — checked 2026-11-17
- **Cross-check** — "SENIAT does not publish the formula; the common mod-11
  method matches live RIFs":
  https://lookuptax.com/validate/venezuela/rif — checked 2026-11-17

Watch out for implementations that give the letter a value and multiply it by 4
(`V=1 ... G=5 × 4`): that is the same thing. A popular variant uses `C = 24`
(letter value 6), which produces a wrong check digit for comunas; it would reject
every one of the comunas migrated from `J` in 2015. The published tables that do
carry the letter, and the migration itself, give `C = J = 12`.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["ven"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("VEN", "12345678")?;
```

## Test numbers (synthetic, not real)

- CI: `12345678`
- RIF: `V-12345678-1` (`V123456781`), `J-12345678-4` (`J123456784`)
- Comuna RIF: `C-12345678-4` (`C123456784`, same base as `J`)

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm (SENIAT modulo 11 rules).

## Notes

- Feature flag: `ven`.
- Country code (ISO 3166-1 alpha-3): `VEN`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- A CI has no check digit, so length and digits are all that can be verified.
- The RIF check digit catches typos; it does not prove the taxpayer is
  registered (SENIAT's RIF lookup is the reference).
