# Check-digit patterns and hand verification

Reference for `identity-validation-algorithms`. It describes the algorithm
families this workspace uses and how to verify one by hand. It deliberately does
not restate each country's weights: the crate README and its validator are the
source for those, and they must match a cited official document.

## Families seen in this workspace

| Family | Shape of the rule | Typical mapping | Examples |
|---|---|---|---|
| Modulus 10, weighted sum | sum of digit × weight, then `(10 - sum % 10) % 10` | — | ECU cédula and natural-person RUC |
| Modulus 10, folding | as above, but each product ≥ 10 has 9 subtracted | — | ECU cédula |
| Luhn | double every second digit from the right, subtract 9 when > 9 | — | DOM cédula |
| Modulus 11, complement | `11 - sum % 11` | `11 -> 0`, `10 -> K` or `10 -> 9` | CHL RUT, ARG CUIT/CUIL, ECU public RUC |
| Modulus 11, remainder | `sum % 11`, sometimes `% 10` | — | SLV NIT legacy table |
| Weight cycling | weights restart (right-to-left `2..7`) or repeat a 7-element set | — | CHL RUT, SLV NIT current table |
| Letter check | final character is a letter, often a mapped modulus result | `10 -> K` | NIC cédula, GTM NIT |
| Alphanumeric check | final character computed over letters too | — | MEX RFC homoclave |

Common rewrite traps, all of which flip correct values:

- **Direction.** Weights read left-to-right versus right-to-left are different
  algorithms. CHL reads the body right-to-left with a cycling 2..7; ECU's cédula
  reads left-to-right. Never reorder one to look like the other.
- **Complement versus remainder.** `11 - sum % 11` and `sum % 11` give different
  digits; the mapping then decides what to do with 10 and 11. Some rules collapse
  those to 0, some to 9, some to a letter.
- **Which slice.** The check digit is often excluded from the sum, but Luhn
  includes every digit. Dominican cédulas are the known trap here.
- **Folding versus doubling.** `p - 9` and `2p` for p ≥ 5 are only equal by
  coincidence on some inputs; use the published operation.
- **Zero padding.** Left-padding a legacy short body changes the weighted sum.
  Keep the original length; do not normalize to the modern length unless the
  agency says the padded form is the canonical one.
- **Separators.** Strip separators before counting and before summing. Do not let
  a separator shift an index: `NonNumeric`/`NonAlphanumeric` report byte offsets
  in the input as typed.
- **Special values.** Reject the all-zero body before computing; many moduli map
  it to 0 and would otherwise accept a placeholder.

## Hand verification recipe

You need one vector per branch (each document kind, each legacy/current path) and
one mutation per vector.

1. Take a synthetic body whose structure is valid.
2. Write the weighted sum out digit by digit on paper (or in the report), apply
   the modulus and the mapping, and record the resulting check character.
3. Append it. That value is now a **derived** test vector.
4. Change one digit of the body and recompute. The new check character must
   differ; if it does not, the algorithm is insensitive to that position and the
   rule is probably wrong.
5. Keep the intermediate sum in the test comment when the rule is unusual
   (folding, letter mapping, two weight tables). It is the cheapest way to make
   the next reviewer believe the vector.

Never copy a value from a tutorial, a gist, a library test suite, an official
example page, a registry export or a live service response. It may belong to a
real person, and it proves nothing about this implementation. The only acceptable
sources are a value you derived here from the published algorithm, or one the
source explicitly labels synthetic — and the second still needs the derivation
checked before it becomes an accepted vector.

## When the source is ambiguous

- If the source gives an example value but not the weights, treat the weights as
  unverified: implement structure only, or reconstruct and label the check
  "reconstructed" in the README with both sources.
- If two official documents disagree, the later one usually supersedes the
  earlier, but only if it says so. Older numbers stay valid under the older rule;
  accept both branches and test both.
- If a rule applies only above or below a cut-off (SLV NIT's correlative, ECU's
  six-digit sequential), implement the cut-off as a named `const` with the
  evidence, and accept the unobserved band only when the alternative is rejecting
  legitimately issued numbers.
