# Evidence matrix template

Copy this table into the task report. One row per document. Fill every column;
write `unknown` rather than leaving a cell blank.

| Document | Local name / kind | Issuer | Norm (level, date) | Shape | Algorithm | Depth | Confidence | Source |
|---|---|---|---|---|---|---|---|---|
| National ID | cédula / `id_card` | Registro Civil | Ley X (2019, art. 4) | 10 digits | mod 10, weights 2..2, fold | full | High | https://… — checked 2026-11-17 |

- **Document**: the document, not the number.
- **Local name / kind**: the name locals use and the `kind` string the crate uses
  (or would use).
- **Issuer**: the body that assigns the number.
- **Norm**: the rule that defines it, with its level and year.
- **Shape**: length, alphabet, separators people type.
- **Algorithm**: check digit or "none published". Not the code.
- **Depth**: full / structure only / unsupported.
- **Confidence**: see below.
- **Source**: primary link with `checked YYYY-MM-DD`; put secondary and
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
