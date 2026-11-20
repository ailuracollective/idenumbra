# Contributing

Contributions to `idenumbra` are welcome. This document summarises the
organisation's conventions (see `.agents/organization-conventions.md`) and the
repository-specific rules (see `AGENTS.md`) that a contributor must follow.
When the two disagree, `.agents/organization-conventions.md` is authoritative.

## Open an issue first

Every change starts as an issue. File issues using the forms in
`.github/ISSUE_TEMPLATE/` (blank issues are disabled). One type per issue,
matching the template's `title:` prefix. The issue must carry `status/ready`
before it can be linked from a pull request.

A need that belongs to a single repository (an extra label, a house colour, a
stricter title length) goes in that repository's own `standards.local.yml`,
not in this repository. Promoting it into the shared standard is a separate
decision, taken in an issue once several repositories want the same thing.

## Branches

`main` is the default branch and is protected. Name branches
`<github-username>/<type>/<description>`, all lowercase, where `<type>` is one
of the title types accepted by `.github/workflows/policy.yml`:

`build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`,
`style`, `test`.

`breaking-change` is a commit marker, not a branch type; use the type of the
underlying change.

```bash
OWNER=$(gh api user -q .login | tr '[:upper:]' '[:lower:]')
git checkout -b "$OWNER/fix/validator-edge-case" main
```

## Commits and pull request titles

Use Conventional Commits. The type is not decoration: the title check reads it,
and the body check compares the body against the template named for it, so
`fix:` is checked against `.github/PULL_REQUEST_TEMPLATE/fix.md`.

- `fix: reject malformed input in ARG CUIT check digit`
- `docs: state that a manifest is a record, not a mechanism`

To start from the right form, open the pull request with `?template=<type>.md`
in the URL. Squash-merge, using the pull request title as the commit message.

## Before opening a pull request

`.github/workflows/policy.yml` checks five things: the branch name, the linked
issue, the type label, the title, and the body's sections. Nothing else is
checked for you. Run the project's checks locally:

```sh
make fmt
make clippy
make test        # or make test-all for the full feature set
```

At minimum, confirm `CODEOWNERS` still works. GitHub skips any line it cannot
parse and silently drops any owner without write access:

```sh
gh api repos/ailuracollective/idenumbra/codeowners/errors --jq '.errors'   # expect []
```

If you edit a pull request template, say which ones in the pull request and
confirm the common headings are unchanged. They are identical across all files,
so any difference makes the gate demand different bodies in different
repositories.

## What a finished change looks like

Merging is the midpoint. A change is finished when:

1. Documentation is updated where public behaviour or configuration changed.
   For this repository that means the affected crate's `README.md` (rules
   validated, editable constants, sources with `checked YYYY-MM-DD`, synthetic
   test values) and the root `README.md` lists, together with the code.
2. All CI checks pass.
3. A change to a catalogue value that a government can change (province or
   department codes, issuer prefixes, reserved codes) is a named `const` at the
   top of the validator file, with a note on when to change it.

## Review

Changes land through a pull request, not a direct push to the default branch.
`.github/CODEOWNERS` assigns reviewers. The last matching pattern wins;
ownership is not additive. Keep a catch-all rule: an unowned path skips the
code-owner gate.

An agent that opens a pull request opens it as a draft.

## Repository-specific rules

- `idenumbra-core` and every country crate are `no_std`; only the orchestrator
  uses `std`. Do not introduce `std` into a country crate.
- Features default to empty. Enable countries explicitly; `all` enables every
  country.
- Each country crate exposes free functions `validate` and `validate_with`, plus
  a zero-sized marker struct implementing `Validator`.
- `validate_with(value, kind)` requires the document kind (`"id_card"`, `"ruc"`,
  `"passport"`, `"dni"`, `"cuit"`, ...). There is no `Option` kind anywhere.
- Catalogue values that a government can change are named `const`s at the top
  of the validator file, with a note on when to change them.

### Synthetic identification numbers

**Synthetic identification numbers only. No exceptions.** Never use a real
person's number, from any source: not an official example page, public
registry, leaked database, tutorial, gist, library test suite, API response,
screenshot, commit, ticket or bug report. Derive test values locally from the
published algorithm, or use a value the source explicitly labels synthetic.
If a user reports a real number, ask for a synthetic equivalent and never copy
the original into code, tests, docs, issues or the final report.

Allowed sources and the acceptance checklist:
`.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.

### Where no public check digit exists

Where no public check digit exists (BOL NIT, COL cédula, CRI cédula/DIMEX,
CUB carné, HND RTN, PAN cédula/RUC), only structure and encoded dates are
validated. Do not invent a check digit for these.

## Labels

`.github/labels.yml` is the manifest. It is a record, not configuration:
GitHub does not read it, and editing it changes no remote. Six families:
`type/`, `status/`, `scope/`, `meta/`, `github_actions`, and `release/`.
`type/` is frozen at five members. Never name a label the manifest does not
declare.
