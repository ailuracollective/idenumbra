---
name: rust-validator-implementation
description: "Use when changing idenumbra's Rust code: implementing or fixing a country validator, adding a document kind, wiring a country into core and std, writing its tests, or updating its crate README. Covers the no_std and MSRV constraints, the Validator API, error variants, feature-gated registration and the lazy registry. Pair with the research and algorithm skills for the rules themselves."
---

# Change the workspace

`AGENTS.md` is the canonical summary of the architecture, conventions and
commands; read it first. This skill is the procedure and the traps. If the two
disagree, follow `AGENTS.md` and then fix this skill.

## Before editing

1. Read `crates/countries/<iso>/src/lib.rs`, its `validators/`, and its README.
   The README states what the crate claims to validate — the tests must assert
   exactly that.
2. Pick the nearest sibling crate and mirror it. The workspace is deliberately
   uniform; a new shape is a review problem, not a feature.
3. Confirm the rules are settled. If not, stop: `identity-regulatory-research`
   then `identity-validation-algorithms` come first.

## Shape of a country crate

- `src/lib.rs`: `#![no_std]`, `const COUNTRY`, `mod validators { include!(…) }`
  one `include!` per file, `pub use validators::{validate, validate_with};`,
  the marker struct (`IdenumbraARG`), `impl Validator`, and the inline
  `#[cfg(test)] mod tests` at the bottom.
- `src/validators/<document>.rs`: one document per file, its catalogue `const`s
  at the top, the check function below.
- `src/validators/validate.rs`: trims, infers the kind from shape, dispatches.
- `src/validators/validate_with.rs`: explicit `kind`, `InvalidFormat` for unknown
  kinds.

Wiring (`core` constant, `std` feature, registry, re-export, README) is a
five-place checklist with the exact edits:
`.agents/skills/rust-validator-implementation/references/crate-wiring.md`.

## Constraints the compiler will not enforce

- **`no_std`.** No `std::`, no `format!`, no `Vec`/`String`, no allocation. Fixed
  stack buffers (`[0u8; 14]`) and `&[u8]` slices. Panics are unacceptable inside
  a validator: no indexing that can go out of bounds on attacker input.
- **No dependencies, no new error variants, no new trait methods.** `idenumbra-core`
  is dependency-free on purpose; the error enum is a shared contract.
- **No `unwrap`/`expect`** outside tests; clippy warns and reviewers reject.
- **Trim once**, in the public entry point, then work on bytes. Keep the byte
  index of the offending character accurate in `NonNumeric`/`NonAlphanumeric`
  (some older validators report `index: 0`; do not copy that).
- **MSRV 1.81, edition 2021.** No newer syntax.
- **Reuse the existing error variants.** `Empty`, `InvalidLength { expected,
  found }`, `NonNumeric`, `NonAlphanumeric`, `CheckDigitMismatch { expected,
  received }`, `InvalidRegion { region }`, `InvalidFormat`,
  `UnrepresentableCheckDigit`, `UnknownCountry`, `ValidatorNotAvailable`.
- **Catalogue data as named `const`s** with a "change when …" note; algorithms
  inline. A government table change must be a one-line edit.
- **Comments only for what the code cannot show**: the rule's source condition, a
  deliberate omission, a legacy branch.

## The API contract

- `validate(value)` infers the kind. `validate_with(value, kind)` requires it.
  There is no `Option` kind.
- The `Validator` trait's default `validate_with` ignores `kind` and delegates to
  `validate`. A crate must override it (as every existing crate does) or typed
  calls silently degrade to inference.
- Country functions trim; the orchestrator does not trim values (it only
  uppercases and trims the country code).
- A disabled country returns `ValidatorNotAvailable`, not `UnknownCountry`.

## Documentation in the same change

- Rustdoc on every public item (`missing_docs` is a warning; do not add to it).
- Crate README: documents, what is and is not validated, editable constants,
  sources with `checked YYYY-MM-DD`, and the synthetic test values. Follow
  `templates/README.template.md`; there is no generator script, so copy and fill
  it, keeping the `{{ }}` TOML escaping.
- A rule change and its README change ship together. A README that disagrees with
  the code is the defect.

## Tests

They live inline in `lib.rs` — there is no `tests/` directory and no
dev-dependency in this workspace. Every test value must be synthetic and derived
in this repository; never paste a number from a bug report, screenshot, registry,
official example or the internet. Allowed sources and the acceptance checklist:
`.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.
What to cover, how to mutate a vector, and the feature-matrix commands:
`.agents/skills/rust-validator-implementation/references/testing.md`.

## Checks

```bash
cargo test -p idenumbra-<iso>              # fast loop, crate alone
make test-all                              # workspace, all features
make fmt                                   # must leave no diff
make clippy                                # workspace, all features, examples
cargo run --example basic --all-features   # orchestrator still resolves codes
```

There is a CI workflow (`.github/workflows/ci.yml`) that runs these on every
push and pull request; `make test-all && make fmt && make
clippy` locally is still the gate. Report the commands you ran and their result.

## Boundaries

- Change one country crate unless the task genuinely spans countries; then report
  the list first.
- Do not regenerate every README, rename kinds, or restructure the workspace as
  a side effect.
- Do not commit unless asked.
