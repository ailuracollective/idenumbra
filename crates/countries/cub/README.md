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

### Editable constants

- `CENTURY_19TH = 9` / `CENTURY_20TH_MAX = 5` (`src/validators/validate.rs`) —
  century code ranges. Edit when a new century is added.

## References

- https://es.wikipedia.org/wiki/Carn%C3%A9_de_Identidad_(Cuba) — checked 2026-11-17
- https://www.cubadebate.cu/especiales/2018/12/20/respuesta-a-matematizando-con-el-carne-de-identidad/ — checked 2026-11-17

No Cuban agency publishes the carné layout or the control-digit algorithm; the
layout above comes from public documentation and the control digit is
deliberately not checked.

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
idenumbra = { version = "0.1", features = ["cub"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("CUB", "12345678")?;
```

## Test numbers (synthetic, not real)

- Carné de identidad: `90010112345` (born 1990-01-01)
- Leap day: `04022912345` (born 1904-02-29)

These values are **synthetic** and do **not** correspond to any real person. Generated locally matching the validator format rules. Source: documented carné de identidad layout.

## Notes

- Feature flag: `cub`.
- Country code (ISO 3166-1 alpha-3): `CUB`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- The control digit is not validated because its algorithm is not published.
