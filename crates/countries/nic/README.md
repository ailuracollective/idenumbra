# idenumbra-nic

Nicaragua identification validator.

## Documents

- Cédula de identidad (13 digits plus a check letter).

## Validation algorithm

Layout: `municipio(3) + birth date DDMMYY(6) + serial(4) + letter`. The letter
is the first 13 digits modulo 23, indexed in
`ABCDEFGHJKLMNPQRSTUVWXY` (23 letters, no `I`, `O` or `Z`); the encoded birth
date must be a real calendar date. Hyphens are ignored. Passports are
validated only when the `passport` kind is requested.

## References

- https://www.cse.gob.ni/ — checked 2026-11-17
- https://www.npmjs.com/package/nicaraguan-dni-validator — checked 2026-11-17
- https://es.wikipedia.org/wiki/Documento_de_identidad_(Nicaragua) — checked 2026-11-17

The CSE does not publish the modulo-23 letter rule; it is implemented by independent validators and by the CSE's own check when issuing.

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

These values are **synthetic** and do **not** correspond to any real person. Generated locally with this crate's validation algorithm. Source: generated locally with the CSE check-letter rule.

## Notes

- Feature flag: `nic`.
- Country code (ISO 3166-1 alpha-3): `NIC`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- Municipality code existence is not checked (the CSE list changes); the check
  letter and birth date are.
