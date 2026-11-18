# idenumbra-{code}

{country} identification validator.

## Documents

{documents}

## Validation algorithm

{algorithm}

### Editable constants

Catalogue values (province codes, issuer prefixes, reserved codes) live in
named `const`s at the top of the validator files. Updating them is a data
change, not an algorithm change:

{constants}

## References

Official sources are listed first; the `implementation` ones are the
cross-checks used where the agency does not publish the check-digit algorithm.

{references}

## Usage

All features:
```toml
[dependencies]
idenumbra = {{ version = "0.1", features = ["all"] }}
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = {{ version = "0.1", features = ["{feature}"] }}
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("{iso}", "12345678")?;
```

## Test numbers (synthetic, not real)

{synthetic}

These values are **synthetic** and do **not** correspond to any real person. Generated locally with this crate's validation algorithm. Source: {source}.

## Notes

- Feature flag: `{feature}`.
- Country code (ISO 3166-1 alpha-3): `{iso}`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
