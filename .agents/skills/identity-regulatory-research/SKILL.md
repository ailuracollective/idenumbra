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
- the abstractable components per document: the fields with published, stable
  semantics that the number encodes (region, taxpayer type, serial, check
  digit, embedded identifier, date, municipality, …), or an explicit
  `none` / `opaque serial only`. A component is only abstractable when the
  issuer publishes its meaning and range; an opaque counter is not;
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
6. **Identify abstractable components.** For each document, list the fields the
   number encodes (region/province, taxpayer or person type, municipality,
   embedded identifier, birth date, century, sex, serial, check digit, …). A
   field is a component only when the issuer publishes its meaning and its
   allowed range; an opaque counter, an unconstrained municipality code or a
   per-document suffix with no published table stays `opaque serial` and is not
   exposed. For each component record the source and confidence the same way as
   for the check digit, because a component with Low evidence must not be
   exposed as a typed field. Note which components are stable (unlikely to
   change) and which are catalogue data the agency can extend (department
   tables, issuer prefixes); the latter become named `const`s, not free text.
   Ecuador (`crates/countries/ecu`) is the reference implementation of this
   step: its `components(value)` exposes `province`, `person_type`, `serial`,
   `check_digit`, `taxpayer`, an embedded `id_card` and a public-entity
   `serial`, and explicitly omits private-company sequentials as ambiguous.
7. **Grade the evidence** (rubric in the template): primary official, official
   secondary, third-party implementation, anecdote. A check digit justified only
   by a third-party implementation is not verified. A component justified only
   by a third-party implementation is likewise not exposed.
8. **Log conflicts and gaps.** Record both sources, both dates, which one you
   followed and why. Record what you could not establish — that list is a
   deliverable, not a failure.
9. **Recommend depth.** Full (algorithm published and still applied), structure
   only (shape/date/region only), or unsupported (nothing verifiable). Say which
   and why, per document.
   Also recommend, per document, whether components are exposed (`components`
   returns `Some`) or `None`, and which fields are omitted and why.

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
- **Survey implementations when a rule is blocked.** Three independent packages
  agreeing on a shape (the Guatemalan CUI department table, the Nicaraguan
  mod-23 letter) is useful evidence that no other interpretation is in
  circulation, and it can show the rule is not implementable with your inputs
  (the Panamanian RUC DV needs the taxpayer type). Record what they do, label
  them third-party, and never let consensus promote a rule to official.
- **Never fill a missing algorithm.** "The agency does not publish a check digit"
  is a finding. Report structure-only instead of inventing weights.
- **Components are claims too.** A field is only a component when the issuer
  publishes its meaning and range; an opaque serial, an unconstrained
  municipality code or a counter with no published table stays `opaque serial`
  and is not exposed. Do not invent a meaning for a digit the agency never
  documented (Ecuador's person type is returned raw for exactly this reason).
- **Components are graded like algorithms.** A component backed only by a
  third-party implementation or by "everyone knows" is Low/None confidence and
  must not be exposed as a typed field; expose it as raw digits or not at all.
- **Components follow the delegation.** When a catalogue (department table,
  issuer prefix, taxpayer-type digit) is "según lo establezca" an agency, cite
  that instrument for the values, not only the law. If the instrument is
  unpublished, the component is not exposed.
- **Serials stay opaque.** A serial counter is not a component even when its
  length is fixed, unless the agency publishes a meaning for it. Do not split an
  opaque run of digits into named fields just because the layout looks regular.
- **Components do not widen the validator.** Exposing a component never adds a
  check the validator did not already perform. If a field's range is not
  validated, it is not a component; it is unchecked structure.
- **Delegated tables: follow the delegation.** When a law says a code, format or
  exception is "según lo establezca" an agency's directorate (RENAP for the
  Guatemalan CUI, DGT for the Costa Rican NITE), the law settles the *shape* and
  the agency's acuerdo, circular or technical annex settles the *values*. Find
  that instrument before deciding a code cannot exist, and cite the instrument,
  not only the law. If it is unpublished, say so; never invent a special code or
  accept every unknown one to be safe.
- **An identifier is not its representation.** Zero padding, a 12-character
  reporting field, an invoice key, a database column or a concatenated type code
  is a format the *consumer* uses. The canonical number is the one the issuer's
  own document format defines; when two agency documents disagree (for example a
  municipal reporting annex saying "NITE 12 caracteres (10 y 2 verificadores)"
  against the tax authority's "10 dígitos" plus zero padding), prefer the one
  scoped to the identifier, quote both, and mark the conflict unresolved.
- **Per-document rules stay per document.** A leading-zero ban, a length or an
  alphabet stated for one document in a shared annex (cédula física, DIMEX) does
  not extend to another (NITE) that the same annex describes without it.
- **A search-index snippet is not the document.** If the PDF is scanned and the
  text cannot be read, the indexed line is *official secondary*: label it that
  way, cite what was read, and keep the rule out of "officially verified".
- **Blocked is not unpublished.** A Cloudflare challenge, a `403` or a scan is an
  access problem, not a finding that the agency publishes nothing. Try a
  text-extraction proxy (for example `https://r.jina.ai/<url>`), the agency's
  alternate host, or a legal gazette copy; cite the official URL and say how the
  text was obtained. Only after those fail may the rule be called unavailable.
- **Read the depuration section.** Tax and registry annexes often specify how to
  clean what the user types (spaces, dots, a hyphen before the check character,
  lowercase `k`). That section decides what the validator must accept, and a
  printed identifier such as `AAAA BBBBB CCCC` is only accepted if it is applied.
- **Quote the operative sentence** in the matrix or README when the wording
  decides a rule (for example "the check digit applies while the sequential has
  up to six digits"). Paraphrase loses the condition.
- **Ask before widening scope.** A reform often affects several documents and
  crates. Report the full impact first; do not silently fix neighbours.

## Report

Finish with the matrix, the depth recommendation, the dated sources, the
conflicts/gaps, the abstractable components per document (or an explicit
`none`/`opaque serial only`), and one line per document on what remains
uncertain. Then choose the next skill:

- rules verified and ready to implement → `identity-validation-algorithms`;
- nothing verifiable → recommend structure-only or no validator, and stop before
  touching code;
- rules changed but code already correct → documentation-only change, no code.
