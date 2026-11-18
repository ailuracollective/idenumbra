# Wiring a country, exactly

Reference for `rust-validator-implementation`. Missing one of these places
compiles but leaves the validator unreachable, or makes the orchestrator tests
fail. `AGENTS.md` lists the same steps in summary; this is the edit list.

Assume `<iso>` is the ISO 3166-1 alpha-3 code (`arg`) and `<ISO>` its uppercase
form (`ARG`), which is also the marker suffix (`IdenumbraARG`). The examples use
`arg`; substitute the code you are actually adding.

## 1. Core constant

`crates/core/src/country.rs`: add the constant and the `canonical` arm.

```rust
/// Country code for ARG.
pub const ARG: &str = "ARG";
```

```rust
} else if code.eq_ignore_ascii_case("ARG") {
    Some(ARG)
}
```

Existing constants have a stray colon ("Country code for ARG:.") — do not
replicate it in new constants.

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
arg = ["dep:idenumbra-arg"]
all = [..., "arg", ...]

[dependencies]
idenumbra-arg = { workspace = true, optional = true }
```

Also add `idenumbra-arg = { version = "0.1.0-alpha.1", path = "crates/countries/arg" }`
to `[workspace.dependencies]` in the root `Cargo.toml`.

## 4. Orchestrator source

`crates/std/src/lib.rs`, four edits:

```rust
// supported_countries()
#[cfg(feature = "arg")]
"ARG",
```

```rust
// registry()
#[cfg(feature = "arg")]
{
    m.insert(
        "ARG",
        (idenumbra_arg::validate, idenumbra_arg::validate_with),
    );
}
```

```rust
// re-export
#[cfg(feature = "arg")]
pub use idenumbra_arg::IdenumbraARG;
```

And the feature test, which has two halves that must both change or the test
reports a count mismatch:

```rust
#[cfg(feature = "arg")]
assert!(sc.contains(&"ARG"));
#[cfg(not(feature = "arg"))]
assert!(!sc.contains(&"ARG"));
```

```rust
let expected = 0usize
    + cfg!(feature = "arg") as usize;
```

## 5. README

`crates/countries/<iso>/README.md`, from `templates/README.template.md`. There is
no generator: copy the template, replace the single-brace placeholders
(`{code}`, `{country}`, `{documents}`, `{algorithm}`, `{constants}`,
`{references}`, `{synthetic}`, `{source}`, `{iso}`, `{feature}`) and keep the
doubled braces in the TOML examples (`{{ version = "0.1.0-alpha.1" }}`) intact.

Also update the root `README.md` crate list, feature list and the
"Validation scope" paragraph when the new country changes them.

## Verification of the wiring

```bash
cargo build --workspace                      # default features: no countries
cargo test -p idenumbra --all-features       # registry + feature test
make test-features FEATURES="arg"            # one feature only
make test-features FEATURES="arg bol"        # subset
cargo run --example basic --all-features
```

`make test-features` runs `cargo test -p idenumbra --features "$(FEATURES)"`, so
it exercises the "country disabled" and "country enabled" branches in the same
suite. If `supported_countries_matches_enabled_features` fails, a `#[cfg]` or the
`expected` sum is out of step with the feature list.
