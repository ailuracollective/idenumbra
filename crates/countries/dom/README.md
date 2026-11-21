# idenumbra-dom

Dominican Republic identification validator.

## Documents

- Cédula (11 digits, Luhn check digit).
- Passport (6-12 alphanumerics).

## Validation algorithm

Cédula: 11 digits validated with the Luhn (mod 10) algorithm over all 11
digits: starting from the check digit (never doubled), every second digit to the
left is doubled, products above 9 have their digits summed (equivalent to
subtracting 9), and the check digit is `(10 - sum mod 10) mod 10`. Equivalently,
the ten payload digits carry weights `1,2,1,2,...` from the left. Hyphens and
spaces are ignored. All-zero numbers are rejected as placeholders.

Depth: structure and check digit only. The first three digits encode the
municipality/registry and the sequence encodes the sex, but the DGII does not
publish a table to validate either, so they are not checked.

### Editable constants

None: this validator has no catalogue-derived constants; every check is algorithmic or structural.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — DGII portal: https://dgii.gov.do/ — checked 2026-11-17. The DGII
  issues the RNC and consumes the cédula, but publishes no check-digit
  specification for it, and no rule describing the cédulas that predate the
  check digit. The JCE is the issuer; neither agency publishes such a list.
- **Implementation** — python-stdnum `do.cedula` (delegates to the standard Luhn
  implementation): https://raw.githubusercontent.com/arthurdejong/python-stdnum/master/stdnum/do/cedula.py
  — checked 2026-11-17
- **Cross-check** — weights `1,2,1,2,...` over the ten payload digits and
  `(10 - sum mod 10) mod 10`:
  https://dgiiapicloud.com/blog/validar-cedula-rnc-ncf-republica-dominicana —
  checked 2026-11-17

Known limit: python-stdnum carries a hard-coded list of 576 issued cédulas that
do **not** match Luhn (checked 2026-11-17; none of them passes the checksum).
Those are real people's numbers, so they cannot be shipped here and, since the
DGII/JCE publish no rule that identifies them by their digits, there is nothing
to validate instead: those cédulas are rejected. This is a deliberate
false-negative policy — the alternative is to accept any 11-digit number.

Inference: anything that is not digits plus `-`/space is treated as a passport
(6-12 alphanumerics), so a cédula mistyped with a letter is reported as a
passport instead of a checksum error.

## Usage

All features:
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["dom"] }
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
