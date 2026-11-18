---
name: identity-validation-algorithms
description: "Use when deciding what a validator should accept and reject: normalization, length and character set, region and date fields, check-digit algorithms and edge mappings, document-kind inference, validation depth, and false positive/negative analysis. Use it to debug a wrong checksum, length, region or date rejection, or to turn a verified official rule into a rule specification. Not for source research or crate wiring."
---

# From specification to check

The research skill answers *what the rule is*. This one answers *what the code
checks*, and what it deliberately does not. A validator that accepts everything
is worse than no validator; so is one that rejects numbers the government issued.

## Output

A rule specification per document, short enough to paste into the crate README:

| Field | Content |
|---|---|
| Normalization | trim, accepted separators and where, case folding |
| Shape | length(s), character set, leading zeros, all-zero rejection |
| Structural fields | region/catalogue codes, encoded dates, reserved codes |
| Check | algorithm, weights, modulus, mappings, which slice it covers |
| Inference | which shapes map to which kind, and which are ambiguous |
| Depth | full / structure only / unsupported, with the reason |
| Omissions | what is intentionally unchecked and why |
| Vectors | synthetic values derived by hand from the algorithm |

No file layout and no Rust: hand the specification to
`rust-validator-implementation`.

## Workflow

1. **Write down current behaviour first.** Read the crate's `validators/` and
   README. Distinguish "the code does X" from "the source says X"; the gap is
   usually the bug.
2. **Normalize.** Trim in every public entry point. Follow the issuing agency's
   own *depuración* rules where it publishes them: SAT removes spaces and dots
   from the NIT and the CUI and a hyphen before the check character, so the
   printed `AAAA BBBBB CCCC` form must be accepted. Accept only the separators
   the document really is written with (dots in ARG DNI, hyphens in ARG
   CUIT, a hyphen right before ECU's last digit, dots/hyphens/spaces in CRI), and
   reject the nearby ones. Decide this before counting characters, and make the
   error index point at the byte the user actually typed.
3. **Fix the shape.** Lengths in use today *and* legacy lengths still in
   circulation. Character set per document. Reject all-zero placeholders
   explicitly: they are never issued. Keep each document's shape rules its own:
   a leading-zero ban or length stated for one document does not transfer to a
   sibling that the same annex describes differently.
4. **Check structural fields.** Region/province/municipality codes against the
   published table; encoded dates as real calendar dates (month 1-12, day valid
   for that month and year). Treat century ambiguity as a documented limitation,
   not a silent assumption. Conventions that are not constraints (sex digit
   parity, issuer prefixes assigned at random) stay unchecked.
5. **Implement the check digit** from the source: weights, direction, modulus,
   complement mapping, digit folding, letter mapping. Confirm which characters
   the sum covers — a check digit computed over the wrong slice passes and fails
   the wrong inputs. Patterns and hand-verification:
   `.agents/skills/identity-validation-algorithms/references/check-digit-patterns.md`.
6. **Decide depth.** Full only when the algorithm is published and still applied.
   When the agency itself stopped guaranteeing it, or publishes no algorithm,
   check structure only and document the omission in the README plus one comment
   above the function. Never invent weights to fill a gap.
7. **Draw the inference boundary.** `validate(value)` may infer only when the
   shapes cannot collide. When two kinds share a shape (CO 10-digit cédula vs
   NIT, PY CI vs RUC by length), infer the common one and require the explicit
   `kind` for the other. Never infer from a passing checksum alone.
8. **Attack your own rule.** List what it accepts that was never issued (false
   positives) and what it rejects that was issued (false negatives): contingency
   prefixes, legacy province codes, 11-digit DIMEX, alphanumeric consecutives,
   historical formats. Fix only what a source justifies.
9. **Derive verification vectors by hand.** Compute the check digit from the
   rule without running the code, then assert both sides. Vectors are synthetic:
   derived here, never a number from the internet, an official example included.
   Allowed sources:
   `.agents/skills/rust-validator-implementation/references/synthetic-test-data.md`.
   See `rust-validator-implementation`'s testing reference for the test cases.

## Constraints

- **Never use a real person's number, even as an illustration.** A number from an
  official example page, registry, tutorial, library test suite, API response,
  screenshot or ticket may belong to someone. Derive it from the algorithm or
  use one the source explicitly labels synthetic, and never quote a reported
  number back.
- **No invented algorithms.** A guessed check digit looks authoritative and
  produces confident false rejections.
- **Matching a library is not evidence.** If a rule is only in python-stdnum,
  the validator may follow it, but the README must say it is a cross-check and
  the tests must not claim official support.
- **Do not loosen a rule until the failing value passes.** Find out whether the
  value is valid under today's rule, a historical rule, or not at all. A
  rejected historical value is a research task, not a code tweak.
- **One rule, one source.** If two sources disagree, the higher norm wins and the
  README records both. Do not average them.
- **Deliberate omissions are documentation, not debt.** A comment such as "the
  SRI stopped guaranteeing this digit" is the correct outcome, not a TODO.
- **Do not add a document kind or a `ValidationError` variant** to make a rule
  fit. Use the existing kinds and variants.

## Report

State per document: what is checked, what is not and why, the synthetic vectors,
and the false positive/negative analysis. If the specification cannot be made
both correct and complete, recommend structure-only and say what evidence would
raise the depth later.
