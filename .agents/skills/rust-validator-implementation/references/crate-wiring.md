# Wiring a country, exactly

Reference for `rust-validator-implementation`. Missing one of these places
compiles but leaves the validator unreachable, or makes the orchestrator tests
fail. `AGENTS.md` lists the same steps in summary; this is the edit list.

Assume `<iso>` is the ISO 3166-1 alpha-3 code (`ecu`) and `<ISO>` its uppercase
form (`ECU`), which is also the marker suffix (`IdenumbraECU`).

## 1. Core constant

`crates/core/src/country.rs`: add the constant and the `canonical` arm.

```rust
/// Country code for ECU.
pub const ECU: &str = "ECU";
```

```rust
} else if code.eq_ignore_ascii_case("ECU") {
    Some(ECU)
}
```

The doc comment on the existing constants has a stray colon ("Country code for
ARG:.") — do not replicate it in new constants.

## 2. Country crate

`crates/countries/<iso>/Cargo.toml` mirrors a sibling: workspace-inherited
metadata, `[lints] workspace = true`, one dependency on `idenumbra-core`. Do not
add `dev-dependencies`. `keywords` and `categories` are crate-specific; there is
no `documentation` line in most crates and publishing is not wired up, so do not
invent one.

Then `src/lib.rs`, `src/validators/{validate.rs,validate_with.rs,<document>.rs}`
and the inline tests. Copy a sibling's `lib.rs` and swap the code, includes,
marker name and tests.

## 3. Orchestrator dependency and feature

`crates/std/Cargo.toml`, three edits:

```toml
[features]
ecu = ["dep:idenumbra-ecu"]
all = [..., "ecu", ...]

[dependencies]
idenumbra-ecu = { workspace = true, optional = true }
```

Also add `idenumbra-ecu = { version = "0.1.0", path = "crates/countries/ecu" }`
to `[workspace.dependencies]` in the root `Cargo.toml`.

## 4. Orchestrator source

`crates/std/src/lib.rs`, four edits:

```rust
// supported_countries()
#[cfg(feature = "ecu")]
"ECU",
```

```rust
// registry()
#[cfg(feature = "ecu")]
{
    m.insert(
        "ECU",
        (idenumbra_ecu::validate, idenumbra_ecu::validate_with),
    );
}
```

```rust
// re-export
#[cfg(feature = "ecu")]
pub use idenumbra_ecu::IdenumbraECU;
```

And the feature test, which has two halves that must both change or the test
reports a count mismatch:

```rust
#[cfg(feature = "ecu")]
assert!(sc.contains(&"ECU"));
#[cfg(not(feature = "ecu"))]
assert!(!sc.contains(&"ECU"));
```

```rust
let expected = 0usize
    + cfg!(feature = "ecu") as usize;
```

## 5. README

`crates/countries/<iso>/README.md`, from `templates/README.template.md`. There is
no generator: copy the template, replace the single-brace placeholders
(`{code}`, `{country}`, `{documents}`, `{algorithm}`, `{constants}`,
`{references}`, `{synthetic}`, `{source}`, `{iso}`, `{feature}`) and keep the
doubled braces in the TOML examples (`{{ version = "0.1" }}`) intact.

Also update the root `README.md` crate list, feature list and the
"Validation scope" paragraph when the new country changes them.

## Verification of the wiring

```bash
cargo build --workspace                      # default features: no countries
cargo test -p idenumbra --all-features       # registry + feature test
make test-features FEATURES="ecu"            # one feature only
make test-features FEATURES="ecu arg"        # subset
cargo run --example basic --all-features
```

`make test-features` runs `cargo test -p idenumbra --features "$(FEATURES)"`, so
it exercises the "country disabled" and "country enabled" branches in the same
suite. If `supported_countries_matches_enabled_features` fails, a `#[cfg]` or the
`expected` sum is out of step with the feature list.
