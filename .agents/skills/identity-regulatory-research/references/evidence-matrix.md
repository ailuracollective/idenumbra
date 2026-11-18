# Evidence matrix template

Copy this table into the task report. One row per document. Fill every column;
write `unknown` rather than leaving a cell blank.

| Document | Local name / kind | Issuer | Norm (level, date) | Shape | Algorithm | Depth | Components | Confidence | Source |
|---|---|---|---|---|---|---|---|---|---|
| National ID | cédula / `id_card` | Registro Civil | Ley X (2019, art. 4) | 10 digits | mod 10, weights 2..2, fold | full | province (code+name), person_type, serial, check_digit | High | https://… — checked 2026-11-17 |

- **Document**: the document, not the number.
- **Local name / kind**: the name locals use and the `kind` string the crate uses
  (or would use).
- **Issuer**: the body that assigns the number.
- **Norm**: the rule that defines it, with its level and year.
- **Shape**: length, alphabet, separators people type.
- **Algorithm**: check digit or "none published". Not the code.
- **Depth**: full / structure only / unsupported.
- **Components**: comma-separated fields with published, stable semantics, or
  `none` / `opaque serial only`. Each must be justified and graded like the
  algorithm. See the `## Components column` section below.
- **Confidence**: see below.- **Source**: primary link with `checked YYYY-MM-DD`; put secondary and
  implementation links in the README, not all of them here.

## Confidence rubric

| Level | Evidence | May justify a check digit? |
|---|---|---|
| **High** | Primary text on the issuer's domain, current, states the algorithm | Yes |
| **Medium** | Another official body restating it, an official notice reproduced elsewhere, or the issuer describes the rule but not the weights | Yes, if labelled "reconstructed" in the README |
| **Low** | Third-party implementation, tutorial, gist, undated page, forum | No — structure only |
| **None** | Only "everyone knows" or an existing code comment with no source | No |

Two independent Medium sources that agree can outweigh one High source that is
silent. Two Low sources never become High.

## Worked example: Ecuador RUC (as recorded in `crates/countries/ecu/README.md`)

| Document | Kind | Issuer | Norm | Shape | Algorithm | Depth | Confidence |
|---|---|---|---|---|---|---|---|
| RUC, natural person | `ruc` | SRI | Reglamento Ley del RUC, Decreto 2167 (updated 2024-02-09), art. 3 | 13 digits, province 01-24, ends `001` | cédula mod 10 over the first 10 + `001` | full | High |
| RUC, public entity | `ruc` | SRI | OECD TIN profile supplied by the SRI | third digit `6`, tenth digit `0` | mod 11 over 8 digits | full | Medium |
| RUC, private company | `ruc` | SRI | SRI comunicado 2020/2021 (quoted reproduction) | third digit `9`, ends `001` | none — mod 11 only while the sequential has ≤ 6 digits; the SRI recommends not validating | structure only | High for the omission, Low for any digit |

The third row is the important one: the *absence* of a reliable check digit is a
High-confidence finding, and it is why `1790012345001` passes even though a naive
mod 11 digit would be `4`.

## Conflict log

Keep it short and decision-oriented:

```text
Source A: <URL> (checked DATE) says <rule>.
Source B: <URL> (checked DATE) says <different rule>.
Followed: A, because <higher norm / later date / issuer's own text>.
Consequence: <what the validator does differently under B>.
```

## Historical transition log

When a format changed, record the old and the new shape and the effective date.
The validator must keep accepting numbers issued under the old rule when they are
still in circulation — that decision belongs in the matrix, not in a guess made
while coding. Examples already documented in this repository: Costa Rica's
cédula jurídica consecutive turning alphanumeric in late 2026, and Argentina's
CUIT/CUIL contingency prefixes 25/26.

## Components column

List the fields the number encodes with published, stable semantics, as a
comma-separated list, or an explicit `none` / `opaque serial only`. Each field
must be justified by a source and graded in the same confidence rubric as the
algorithm. A field whose range is not validated is not a component (it is
unchecked structure, and must not be exposed). Examples:

- Ecuador cédula: `province (code+name), person_type, serial, check_digit`.
- Ecuador RUC (natural person): `province, taxpayer, id_card (embedded
  cédula)`.
- Ecuador RUC (public entity): `province, taxpayer, serial (5 digits)`.
- Ecuador RUC (private company): `province, taxpayer` (serial omitted: 6 vs 7
  digits cannot be split from the check digit).
- Guatemala CUI: `correlativo (8), check_digit, department (code), municipality
  (code)`.
- Nicaragua cédula: `municipality (code), birth_date, serial, check_letter`.
- Cuba carné: `birth_date (YYMMDD), century_code, serial, sex` (sex is
  published but not a constraint, so it is returned raw).
- Argentina DNI: `none` (no published structure beyond length).
- Argentina CUIT: `prefix (taxpayer type), dni (8), check_digit` (prefix is
  assigned at random, so it is returned raw and never enforced).
- Brazil CPF: `region_code (state), base, check_digit (×2)`.
- Brazil CNPJ: `root (8), branch (4), check_digit (×2)`.

Catalogue fields (department tables, issuer prefixes, taxpayer-type digits) are
listed here too, but become named `const`s in the crate, not free text. An
opaque serial counter is never listed, even when its length is fixed.
