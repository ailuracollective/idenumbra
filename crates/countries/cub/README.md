# idenumbra-cub

Cuba identification validator.

## Documents

- Carné de identidad (11 digits).
- Passport (6-12 alphanumerics).

## Validation algorithm

Carné de identidad: 11 digits laid out as birth date `YYMMDD`, a century code
(`9` = 19th century, `0-5` = 20th, `6-8` = 21st), a 2-digit serial, the sex
digit and a control digit. The encoded birth date is checked as a real
calendar date. The control digit algorithm is not public, so it is not
verified.

The sex digit (10th) is even for men and odd for women, but every digit is
even or odd, so that position cannot reject anything without knowing the
person's registered sex; the MININT reassigns the whole number when the
registered sex changes. The two middle serial digits are arbitrary (`00`-`99`).

Ley 188/2026, *Del Sistema de Identidad Personal y Domicilio* (Gaceta Oficial
No. 67 Ordinaria de 2026), keeps the "número de identidad permanente" unique,
irrepetible and in principle invariable, and adds a digital identity; it does
not publish a new number format. It was due to enter into force 120 days after
publication and its reglamento is still pending, so the 11-digit layout above
is the one in circulation.

### Editable constants

- `CENTURY_19TH = 9` / `CENTURY_20TH_MAX = 5` (`src/validators/validate.rs`) —
  century code ranges. Edit when a new century is added.

## References

- **Official** — Gaceta Oficial No. 43 Extraordinaria de 16/10/2014, MININT
  Resolución No. 14 (new carné; the anexo lists the permanent identity number
  among the card data):
  https://www.gacetaoficial.gob.cu/sites/default/files/go_x_43_2014.pdf — checked 2026-11-17
- **Official** — Gaceta Oficial No. 67 Ordinaria de 2026, Acuerdo X-180
  (Ley 188 "Del Sistema de Identidad Personal y Domicilio"):
  https://www.gacetaoficial.gob.cu/es/acuerdo-x-180-de-2026-de-asamblea-nacional-del-poder-popular
  — checked 2026-11-17
- **Official** — Presidencia de la República, on the Ley 188 project:
  https://www.presidencia.gob.cu/es/noticias/proyecto-de-ley-del-sistema-de-identidad-personal-y-domicilio-una-norma-juridica-necesaria-y-actualizada/
  — checked 2026-11-17
- **Implementation** — https://es.wikipedia.org/wiki/Carn%C3%A9_de_Identidad_(Cuba) — checked 2026-11-17
- **Implementation** — Cubadebate, "Matematizando con el carné de identidad"
  (digit-by-digit layout; the author states the control-digit algorithm is not
  public): http://www.cubadebate.cu/especiales/2018/12/17/matematizando-con-el-carne-de-identidad/
  — checked 2026-11-17

No Cuban agency publishes the carné layout, the sex-digit convention or the
control-digit algorithm. The layout above comes from public documentation, and
the control digit is deliberately not checked.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["cub"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("CUB", "12345678")?;
```

## Test numbers (synthetic, not real)

- Carné de identidad: `90010112345` (born 1990-01-01)
- Leap day: `04022912345` (born 1904-02-29)
- 21st century: `20010180110` (born 2020-01-01)

These values are **synthetic** and do **not** correspond to any real person. Generated locally matching the validator format rules. Source: documented carné de identidad layout; century codes from the published digit-by-digit description.

## Notes

- Feature flag: `cub`.
- Country code (ISO 3166-1 alpha-3): `CUB`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The control digit is not validated because its algorithm is not published.
- The sex digit is not validated: even/odd is a convention, not a constraint.
