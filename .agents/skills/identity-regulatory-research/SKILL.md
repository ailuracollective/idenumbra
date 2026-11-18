---
name: identity-regulatory-research
description: "Use when you must establish what an identification document is and which rules are authoritative today: inventorying a country's documents, auditing existing rules, investigating a rejected historical value, or reacting to a regulatory change. Produces a dated evidence matrix with confidence levels and an explicit list of unverifiable claims. Not for editing Rust code."
---

# Establish the authoritative rules

idenumbra accepts or rejects numbers, so every rule is a claim about another
country's law. This skill gets that claim to a named source and a date before
anyone writes code.

## Output

An evidence matrix (template:
`.agents/skills/identity-regulatory-research/references/evidence-matrix.md`)
with one row per document a caller could plausibly pass, plus:

- the recommended validation depth per document: full, structure only, or
  unsupported;
- the sources used, each with the date you opened it;
- open questions and conflicts, with what each answer would change.

No code, no README rewrite, no opinion about Rust. Hand the verified rows to
`identity-validation-algorithms`.

## When to use

- A new document type or a new country.
- Auditing or revalidating the rules of an existing crate.
- A value the government issued is rejected (historical format).
- A reform, new format or new document type is announced.
- A crate README states a rule that cannot be traced to a source.

## Inputs

- The country (ISO 3166-1 alpha-3) and the document names as locals say them.
- Existing claims: `crates/countries/<iso>/README.md` and its validators, if the
  crate exists. Start there — most of the research is usually already done and
  cited.
- Affected values, if the task came from a bug. Never handle a real person's
  number: if one was reported, ask for a synthetic equivalent, derive one from
  the algorithm, and do not repeat the original in the matrix, README, tests or
  report.

## Workflow

1. **Read what the repository already claims.** The crate README lists documents,
   depth, constants and sources. Note which claims have a source and which are
   inherited from code comments.
2. **Name the issuing authority.** The registry or tax administration that
   assigns the number, not a bank, school, form vendor or Wikipedia. The crate
   README's References section already names it for most countries.
3. **Climb the hierarchy.** Law → reglamento/decreto/acuerdo → agency
   resolution/circular → technical annex (ficha técnica, anexo, layout de
   comprobantes). The higher norm wins. Agency technical docs are the best
   evidence for *how* a number is validated, even when a law defines the format.
4. **Date every rule.** Find the effective date and what it replaced. A rule is
   current only if a source issued after the last reform says so. Search for
   `<document> nuevo formato`, `reforma`, `<document> <year>`, and check the
   agency's news/circular pages.
5. **Separate identifiers from serials.** A validator checks the identifier and
   its published check digit. Issue dates, serial counters, municipality codes
   that are not constrained, and per-document suffixes are only checkable when
   the agency publishes the table — otherwise they are structure, or nothing.
6. **Grade the evidence** (rubric in the template): primary official, official
   secondary, third-party implementation, anecdote. A check digit justified only
   by a third-party implementation is not verified.
7. **Log conflicts and gaps.** Record both sources, both dates, which one you
   followed and why. Record what you could not establish — that list is a
   deliverable, not a failure.
8. **Recommend depth.** Full (algorithm published and still applied), structure
   only (shape/date/region only), or unsupported (nothing verifiable). Say which
   and why, per document.

## Rules

- **Synthetic numbers only, no exceptions.** A number found on the internet is
  not a test fixture: official examples, public registries, tutorials, gists,
  library test suites, API responses and bug reports can all contain a real
  person's number. Derive values locally from the algorithm, or use one the
  source explicitly labels synthetic. Never quote a reported number back. The
  allowed-source list is in
  `.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.
- **Government-first.** Prefer the issuer's own domain. An official PDF, annex or
  resolution beats a summary of it; a summary by another state body is
  acceptable *secondary* evidence and must be labelled as such.
- **Every rule carries a URL you opened in this session and its date.** `- URL —
  checked YYYY-MM-DD`, the format the crate READMEs already use. A source you
  did not open is not evidence, however familiar it feels.
- **A current rule needs a current source.** A 2019 page is evidence about 2019.
  If the only source predates a known reform, say so and lower the confidence.
- **Withdrawn pages** may be cited through `web.archive.org`, labelled as
  archived, and never as the current rule without a live confirmation.
- **Libraries are cross-checks, never authority.** python-stdnum and similar
  projects can point at an algorithm; they cannot make it official. If a rule
  exists only in a library, it is Low confidence and not enough for a check
  digit.
- **Never fill a missing algorithm.** "The agency does not publish a check digit"
  is a finding. Report structure-only instead of inventing weights.
- **Quote the operative sentence** in the matrix or README when the wording
  decides a rule (for example "the check digit applies while the sequential has
  up to six digits"). Paraphrase loses the condition.
- **Ask before widening scope.** A reform often affects several documents and
  crates. Report the full impact first; do not silently fix neighbours.

## Report

Finish with the matrix, the depth recommendation, the dated sources, the
conflicts/gaps, and one line per document on what remains uncertain. Then choose
the next skill:

- rules verified and ready to implement → `identity-validation-algorithms`;
- nothing verifiable → recommend structure-only or no validator, and stop before
  touching code;
- rules changed but code already correct → documentation-only change, no code.
