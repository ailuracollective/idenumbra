---
name: country-validator
description: "Use when adding a new idenumbra country validator or re-validating an existing one: research the official identification documents and check-digit rules, confirm the rules are current as of today, decide how deep validation can go without inventing algorithms, then implement or audit the crate, core constants, std registry, tests and README."
---

# Research and validate a country

idenumbra validates identification numbers for one country per crate. Two modes:
**add** (`crates/countries/<iso>/` does not exist) and **revalidate** (it does).
Both start with the same research pass; only the writing changes.

## Rules that override everything

- **Today's rules, not remembered ones.** Every rule you write must come from a
  source you actually read in this session, and that source must describe the
  document as issued now. A page older than a known reform is stale evidence:
  search for `<document> nuevo formato`, `<document> reforma`, `check digit
  <country> <year>`, and prefer the issuing agency's own domain over mirrors.
  Record when you checked it: `- https://… — checked YYYY-MM-DD`.
- **No invented check digits.** When the agency publishes no rule, validate only
  structure and encoded dates, and say so in the README plus one comment above
  the function. Precedents: CO cédula, CR cédula/DIMEX, CU carné, PA
  cédula/RUC. A validator that accepts everything is worse than none: if nothing
  is verifiable, report that instead of adding the crate.
- **Synthetic numbers only.** Generate test values from the algorithm you wrote.
  Never copy a number from a tutorial, gist, or example response: it may belong
  to a real person. State in the README that values were generated locally.
- **`no_std`, no dependencies, ISO 3166-1 alpha-3**, one crate per country.
  Catalogue data (province tables, issuer prefixes, reserved codes) lives in
  named `const`s with a "change when …" note; algorithms stay inline.
- Do not commit unless asked.

## 1. Inventory the documents

List every document a caller could plausibly pass: national ID, tax ID,
passport, and the aliases people use. Map each to an existing `kind` string —
`id_card`, `passport`, plus the country's own term where that is the real name
(`ruc`, `cuit`, `dni`, `nit`, `rut`). No `Option` kind, no new enum.

Record before writing code:

| Document | Kind | Shape | Check digit | Source |
|---|---|---|---|---|

- Shape: lengths, alphabet (digits only? `K`? letters?), separators people type.
- Check digit: algorithm, weights, mappings (`10 -> K`, `11 -> 0`), and whether
  the agency itself publishes it.
- Provenance: official agency page/PDF first. Wikipedia, python-stdnum and other
  libraries are cross-checks, never the only evidence; if one is, say so in the
  README.

## 2. Decide the depth of validation

Per document: full validation, structure only, or declared unsupported. Then
look at the input as it really arrives — trim, case, dots, hyphens, spaces,
leading zeros. Match the behaviour of the nearest existing crate (ECU, for
example, strips a hyphen only right before the last digit and rejects it
anywhere else).

## 3. Implement (add mode)

Mirror an existing crate such as `crates/countries/ecu`:

- `src/lib.rs`: `#![no_std]`, `mod validators { include!("validators/…") }`,
  `pub use`, marker struct, `impl Validator`, unit tests at the bottom.
- `src/validators/<document>.rs`: one document per file, with the `const` tables
  it owns at the top.
- `src/validators/validate.rs`: `validate` — trim, infer the kind from shape and
  length, fall back to passport when the input is alphanumeric.
- `src/validators/validate_with.rs`: `validate_with(value, kind)` — explicit
  kind, `InvalidFormat` for unknown ones.
- Map failures to existing `ValidationError` variants: `Empty`,
  `InvalidLength { expected, found }`, `NonNumeric { index }`,
  `NonAlphanumeric { index }`, `CheckDigitMismatch { expected, received }`,
  `InvalidRegion { region }`, `InvalidFormat`, `UnrepresentableCheckDigit`.
  Do not add a variant for one country.

Wire it in — all five places, or the crate compiles but is unreachable:

1. `crates/core/src/country.rs`: `pub const XXX` and a `canonical` branch.
2. `crates/countries/<iso>/Cargo.toml`, metadata identical to siblings.
3. `crates/std/Cargo.toml`: optional dependency, feature, and `<iso>` inside the
   `all` feature list.
4. `crates/std/src/lib.rs`: the `#[cfg(feature = "…")]` entry in
   `supported_countries()`, the `registry()` insert, the re-export, and the
   `supported_countries_matches_enabled_features` test (both the
   `contains` assertion and the `expected` sum).
5. `crates/countries/<iso>/README.md` from `templates/README.template.md`,
   keeping the `{{ }}` escaping intact.

Tests worth having per document: a passing value, a wrong check digit (assert
the exact `expected`/`received`), a bad structure, length or region, the
separators you accept and the nearby ones you reject, and every `kind` through
`validate_with` including a mismatch (`id_card` value validated as the tax ID).
Cover the interesting edges: check letter (`K`), leading zeros, a `10 -> K` or
`11 -> 0` mapping.

## 4. Revalidate (existing crate)

Same research pass, but audit instead of rewrite. Read the crate, then compare
rule by rule:

- Do the `const` tables still match the agency's current list (provinces,
  municipalities, issuer prefixes, reserved codes)? Commented counts like "INE
  currently lists 340" age fastest.
- Does the algorithm match the source, including edge mappings and modulus?
- Are the kinds still the ones people use, and is inference still right for the
  shapes in circulation?
- Do the tests still pass and still assert what the README claims?
- Is anything accepted that should not be, or rejected that clearly should not?

Report first: findings ranked with `file:line` and the source that disagrees.
Then make the minimal fix. Never change an algorithm or add a document kind on a
hunch — the evidence goes in the README reference line.

## 5. Verify

```bash
cargo test -p idenumbra-<iso>              # crate alone, fast loop
make test-all                              # workspace, all features
make clippy && make fmt                    # no unwrap/expect, docs on public items
cargo run --example basic --all-features   # orchestrator still resolves the code
```

`no_std` matters: no `std::`, no `format!`, no allocation in country crates.

## 6. Report

Finish with: documents covered and depth (full / structure-only), what is not
validated and why, sources with access date, the synthetic test numbers used,
and anything still doubtful. Ask before touching other crates or the README
template.
