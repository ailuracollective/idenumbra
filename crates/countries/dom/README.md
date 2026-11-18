# idenumbra-dom

Dominican Republic identification validator.

## Documents

- Cédula (11 digits, Luhn check digit).
- Passport (6-12 alphanumerics).

## Validation algorithm

Cédula: 11 digits validated with the Luhn (mod 10) algorithm; digits are
doubled from the right (the 10 payload digits), products above 9 have 9
subtracted, and the check digit is `(10 - sum mod 10) mod 10`. Hyphens and
spaces are ignored.

## References

- https://dgii.gov.do/ — checked 2026-11-17
- https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/do/cedula.py — checked 2026-11-17
- https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Rep%C3%BAblica_Dominicana) — checked 2026-11-17

The DGII does not publish the Luhn rule; it is cross-checked against python-stdnum and the DGII online validator.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["dom"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("DOM", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula: `123-4567890-3` (`12345678903`)

These values are **synthetic** and do **not** correspond to any real person. Generated locally with this crate's validation algorithm. Source: generated locally with the Luhn algorithm used by the DGII.

## Notes

- Feature flag: `dom`.
- Country code (ISO 3166-1 alpha-3): `DOM`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
