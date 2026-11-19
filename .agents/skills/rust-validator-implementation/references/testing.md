# Testing validators

Reference for `rust-validator-implementation`. Tests here are inline
`#[cfg(test)] mod tests` blocks at the bottom of each crate's `lib.rs`, using
`use super::*;`. There is no `tests/` directory and no dev-dependency in the
workspace, so no `proptest`, `quickcheck` or `criterion`.

## What every document needs

| Case | Assertion |
|---|---|
| Accepted value | `is_ok()`, at least one per branch (each kind, legacy/current table) |
| Wrong check digit | the exact `CheckDigitMismatch { expected, received }` |
| Bad shape | the exact `InvalidLength`/`NonNumeric`/`NonAlphanumeric`/`InvalidFormat` |
| Bad region/date | the exact `InvalidRegion { region }` or `InvalidFormat` |
| All-zero placeholder | rejected, and named as a placeholder in the test |
| Separators | each accepted form passes; the nearby rejected form fails |
| Kinds | every kind accepts its own value and rejects a value of another kind |
| Deliberate omission | the value that a naive check digit would reject is `is_ok()` |

Assert errors, not just `is_err()`. The exact variant and payload are the contract
callers match on, and a test that only checks failure passes when the reason
changes.

## Naming and comments

Name tests for the behaviour, as the existing crates do
(`accepts_id_card_and_rucs`, `rejects_bad_cuit`, `honours_document_kind`). When a
value exists to pin a non-obvious rule, put the rule in a comment: the legacy
province `50`, the contingency prefixes, the check letter `K`, the modulus being
deliberately unchecked. A future reader must not be able to "fix" a correct test.

## Mutation, not duplication

For each accepted vector, mutate exactly one position and assert the failure:

- change a body digit → recomputed check digit differs, so
  `CheckDigitMismatch { expected, received }`;
- change the check digit → `expected` stays, `received` is the new one;
- drop or add a character → `InvalidLength`;
- change a region prefix → `InvalidRegion`;
- replace a digit with a letter (and vice versa) → the character-set error.

A check that stays correct under body mutation is not reading that position; that
is a defect, and the mutation test is how you catch it.

## Derived vectors

Derive the value from the algorithm by hand (see the algorithm skill's
check-digit reference) and keep the intermediate sum in a comment when the rule
is unusual. Values in the README's synthetic section and in the tests must agree.

Never take a value from a tutorial, gist, library test suite, live lookup,
official example page, registry export, screenshot or bug report. If a source
labels a value synthetic, it is still only a candidate: re-derive it before
accepting it. There is no situation in which pasting a number of unknown
provenance into a test is acceptable. Allowed sources:
`.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.

## A reported real number

A bug report may contain the number the reporter is trying to validate. That is a
real person's data:

1. Do not copy it into the test, the issue, the commit message or the report.
2. Reproduce the defect with a synthetic value of the same shape (same length,
   same prefix class, same failing branch).
3. If the bug depends on a value that cannot be reconstructed synthetically, say
   what is missing and ask the reporter for a masked equivalent — do not use the
   original.
4. If the number is already committed, replace it and note the swap in the commit
   message, without repeating the value.

## Dates and boundaries

Encode dates with a real calendar: month 0 and 13, day 0, day 32, 29 February in
a leap and a non-leap year, and the century boundary the crate assumes. Where the
year is two digits, the century choice is a documented limitation — test the
assumption, do not hide it.

## Structure-only validators

When no check digit is published, the tests must prove the depth, not pretend it
is deeper:

- a well-formed value passes, including one whose last digit a naive checksum
  would reject;
- a malformed value of the same length fails;
- the README's "what is not validated" list matches the assertions.

## Historical values

When a format changed, keep a test for the old rule next to the new one, each
with the effective date in a comment. A regression here usually means a legacy
branch was removed as "dead code".

## Feature matrix and `no_std`

```bash
cargo test -p idenumbra-<iso>                 # crate alone
cargo test --workspace --all-features         # every country
make test-features FEATURES="arg"             # one enabled feature
make test-features FEATURES="arg bol"         # subset
cargo build --workspace                       # default features: nothing enabled
```

`make test-features` is what exercises the `#[cfg(not(feature = …))]` half of the
orchestrator tests. CI (`.github/workflows/ci.yml`) runs fmt, clippy and the
test suite on every push/PR; there is no `no_std` cross-target build here, so
`no_std` compliance is enforced by `#![no_std]` plus review: no `std::`,
`format!`, `Vec` or `String` in a country crate. `cargo build` alone does not
prove it.

## When a test fails

1. Reproduce with the smallest synthetic value and the exact error.
2. Decide which side is wrong: the rule (research/algorithm skill), the code, or
   the test's assumption. Do not adjust the expected value to make the suite pass
   — that is how a wrong algorithm becomes permanent.
3. Fix once in the shared path, re-run the crate, then the workspace.
4. Report the root cause, not just "tests green".

Property-based and differential testing are not available today. If a task
genuinely needs them, propose adding a dev-dependency explicitly; do not sneak
one into a validator PR. Until then, the mutation pass above is the substitute,
and a published official example vector is the only acceptable "differential"
reference.
