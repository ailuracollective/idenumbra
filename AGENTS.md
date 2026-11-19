# AGENTS.md

Guidance for coding agents working in this repository.

## Project

`idenumbra` is a Cargo workspace of Latin American identification validators.

- `crates/core`: shared contract (`Validator` trait, `ValidationError`,
  country code constants). Dependency-free and `no_std`.
- Country validators: `crates/countries/{arg,bol,bra,chl,col,cri,cub,dom,ecu,gtm,hnd,mex,nic,pan,per,pry,slv,ury,ven}` (ISO code only; package name remains `idenumbra-<code>`).
  Each is `no_std` and implements `idenumbra_core::Validator`.
- `crates/std`: orchestrator with one Cargo feature per country plus `all`.
  Builds a lazy `BTreeMap` registry on first use.

Country codes and feature names use ISO 3166-1 alpha-3 (e.g. `ARG`) and are matched case-insensitively.

## Layout

- `Cargo.toml`: workspace root, shared package metadata, workspace lints.
- `crates/`: all crates.
- `templates/README.template.md`: template for per-crate READMEs.
- `.agents/skills/`: one directory per agent skill (`SKILL.md` +
  `references/`), discovered by OpenCode and Claude-compatible agents.
- `lefthook.yml`: Git hooks (pre-commit fmt/clippy, commit-msg commitlint).
- `.commitlintrc.yaml`: conventional commit rules for commitlint-rs.
- `Makefile`: common build/test/lint targets.

## Build / Test / Lint

From the repo root:

```bash
make build        # cargo build --workspace
make build-all    # cargo build --workspace --all-features
make test         # cargo test --workspace
make test-all     # cargo test --workspace --all-features
make test-features FEATURES="arg"
make fmt          # cargo fmt --all
make clippy       # cargo clippy --workspace --all-features --examples
make doc          # cargo doc --workspace --all-features --no-deps
make example      # cargo run --example basic --all-features
make hooks        # lefthook install (pre-commit: fmt + clippy)
make clean
```

Single country crate without the orchestrator:

```bash
cargo test -p idenumbra-arg
```

## Releases

Automated by Google's release-please (`.github/workflows/release-please.yml`).
It runs on `main`, opens a `chore: release X.Y.Z-alpha.N` PR from the
conventional commits, and on merge tags `vX.Y.Z-alpha.N` and writes
`CHANGELOG.md`.

The project is in alpha: `versioning: prerelease` + `prerelease-type: alpha` +
`prerelease: true` keep every version a prerelease (`0.1.0-alpha.1`, then
`0.1.0-alpha.2`, ...) and mark the GitHub release as a prerelease. To cut a
stable version, add a `Release-As: 0.1.0` commit (or remove those three
settings); nothing becomes stable on its own. The `release-as` pin in the
config only exists to bootstrap `0.1.0-alpha.1`: it overrides the computed
version, so remove it once that tag exists or every later release repeats it.

One version for the whole workspace: `release-please-config.json` bumps
`[workspace.package].version`, every `[workspace.dependencies]` version, the
lockfile and `version.txt`. It uses `release-type: simple` + `extra-files`
because the root manifest is a virtual workspace (the `rust` strategy and the
`cargo-workspace` plugin both require a `[package]` section there).
Publishing to crates.io is not wired up.

## Conventions

- Rust 2021 edition, `rust-version = 1.81`, license MIT.
- Workspace lints: `unsafe_code = "forbid"`, `missing_docs = "warn"`,
  clippy `unwrap_used`/`expect_used = "warn"`. New public items need docs;
  avoid `unwrap`/`expect` in non-test code.
- `idenumbra-core` and every country crate are `no_std`; only the orchestrator
  uses `std` (for `OnceLock`/`BTreeMap`).
- Features default to empty. Enable countries explicitly; `all` enables every
  country. A disabled country is not compiled and returns
  `ValidationError::ValidatorNotAvailable`.
- Each country crate exposes free functions `validate` and `validate_with`, plus
  a zero-sized marker struct (e.g. `IdenumbraARG`) implementing `Validator`.
- `validate_with(value, kind)` requires the document kind, e.g. `"id_card"`,
  `"ruc"`, `"passport"`, `"dni"`, `"cuit"`; `validate(value)` infers it from the
  input. There is no `Option` kind anywhere: pick the function you mean.
- Registry is built lazily and cached; do not add eager initialization.
- Keep changes minimal; add a small test for new non-trivial validation logic.
- Catalogue values that a government can change (province/department codes,
  issuer prefixes, reserved codes) are named `const`s at the top of the
  validator file, with a note on when to change them, and each crate README
  lists them with an official source. Algorithms stay inline.
- **Synthetic identification numbers only. No exceptions.** Never use a real
  person's number, from any source: not an official example page, public
  registry, leaked database, tutorial, gist, library test suite, API response,
  screenshot, commit, ticket or bug report. Derive test values locally from the
  published algorithm, or use a value the source explicitly labels synthetic.
  If a user reports a real number, ask for a synthetic equivalent and never copy
  the original into code, tests, docs, issues or the final report. Allowed
  sources and the acceptance checklist:
  `.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.
- Each crate README states, per document, what is validated and what is not, the
  editable constants, the sources with `checked YYYY-MM-DD`, and the synthetic
  test values. Code and README change together.
- Per-crate READMEs follow `templates/README.template.md` by copying it; there is
  no generator. Replace the single-brace placeholders and keep the `{{ }}` TOML
  escaping.

## Agent skills

Skills live in `.agents/skills/<name>/SKILL.md`, with supporting files under
`references/` that are read only when the task needs them. Country-specific
details stay in `crates/countries/<iso>/README.md`, not in a skill.

| Task | Skill |
|---|---|
| Establish what a document is and which rules are authoritative today; historical formats, regulatory changes, a country audit inventory | `identity-regulatory-research` |
| Decide what to check: normalization, check digit, region/date fields, inference, validation depth; debug a wrong checksum or rejection | `identity-validation-algorithms` |
| Edit Rust: implement or fix a validator, add a kind, wire a country, write tests, update a crate README | `rust-validator-implementation` |

Normal flow: research → algorithms → implementation. This file stays the
canonical summary of architecture, conventions and commands; skills hold the
procedure and the traps. If a skill disagrees with this file, follow this file and
fix the skill.

## Known traps

- The `Validator` trait's default `validate_with` ignores `kind` and delegates to
  `validate`. Kind-aware callers should use the free `validate_with` functions or
  `Idenumbra::validate(..., kind)`; `Idenumbra::infer(...)` is the untyped entry.
- Country `validate` functions trim input; the orchestrator uppercases the country
  code but does not trim the value.
- `missing_docs` is a warning, not an error, but keep docs complete for public APIs.
- Test values are derived from the algorithm in this repository, never copied
  from a person. The synthetic-numbers rule under Conventions is absolute.
- Where no public check digit exists (BOL NIT, COL cédula, CRI cédula/DIMEX, CUB
  carné, HND RTN, PAN cédula/RUC), only structure and encoded dates are validated.
  Do not invent a check digit for these.
- Colombian 10-digit numbers are validated as cédulas (which have no check
  digit); the NIT check digit is only checked with `validate_with(.., "nit")`.
  Paraguayan 6-7 digit numbers are CIs and 8-9 digit numbers are RUCs, so use
  the `ci`/`ruc` kinds when the document type is known.
- Guatemalan NIT check characters may be `K`; NIC cédulas end with a check letter,
  not a digit.
- ARG CUIT/CUIL prefixes are deliberately not enforced: 25/26 are valid and
  ARCA assigns new prefixes randomly. GTM CUI checks municipality counts per
  department (INE currently lists 340); update the table when one is created.
- Per-crate READMEs are copies of `templates/README.template.md`, not generated
  output: there is no script that renders them. Keep the `{{ }}` TOML escaping in
  the template, and update the root `README.md` lists when a country is added.
- `.github/workflows/ci.yml` runs fmt, clippy and the test suite (default and
  all features) on every push and pull request. `.github/workflows/publish.yml`
  re-runs the same checks on a tagged tree (plus tag/version verification) and
  then publishes every crate to crates.io on a published release.
  `make test-all && make fmt && make clippy` locally is still the gate.
- Git hooks require `lefthook` and `commitlint` (`cargo install commitlint-rs`)
  on `PATH`; run `make hooks` once per clone.

## Adding a country

1. Add the code constant and `canonical` arm in
   `crates/core/src/country.rs`.
2. Create `crates/countries/<code>/` with `Cargo.toml` (depends on
   `idenumbra-core`), `src/lib.rs` (`#![no_std]`, `validators/` module with
   one file per validation function, `validate`, `validate_with`,
   marker struct + `Validator` impl), README and tests.
3. Add the dependency/feature in `crates/std/Cargo.toml`.
4. Add the `#[cfg(feature = ...)]` entries in `supported_countries()` and the
   registry in `crates/std/src/lib.rs`, plus the re-export.
5. Add the country to the `all` feature list and to the orchestrator tests if
   appropriate.
6. Add the README from `templates/README.template.md` and update the root
   `README.md` lists.

The exact edit list (all five wiring places, including the two halves of the
feature test) is in
`.agents/skills/rust-validator-implementation/references/crate-wiring.md`.

## Do not commit unless asked.
