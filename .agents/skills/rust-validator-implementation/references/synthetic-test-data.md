# Synthetic test data: allowed sources

Reference for every skill. A test value is data about a person only if someone
made it so. This file lists where values may come from and what to do when one
arrives from the outside.

## The rule, without exceptions

Never use a real person's identification number, from any source. Not an
official example page, not a public registry, not a leaked database, not a
tutorial, gist, library test suite, API response, screenshot, commit, ticket or
bug report. A number that comes with a name attached, or that a site resolves to
a person, is someone's data.

A value is acceptable only if it was **authored here from the published
algorithm**, or if its source **explicitly labels it synthetic, fictitious or
reserved**. "It looked fake" is not provenance.

## Tier 1 — derive it here (default)

Take a valid structure and a fictional seed, then compute the check digit with
the algorithm the crate implements:

- sequential bodies: `12345678`, `1234567890101`, `001-010190-0001`;
- names chosen as placeholders: `GALA`, `LOPJ`, `GODE`, `ABC`;
- fixed dates: `1990-01-01`, `2004-02-29`;
- a legacy/current branch each, plus one mutation per vector.

Record the seed and the intermediate sum in the test comment, and say in the
README that the values were generated locally. This is the only tier that gives
a complete provenance story: nobody's data was read to produce it.

A derived number can coincide with an issued one in theory — any format-valid
number can. That is why the value is labelled synthetic everywhere, is never
looked up against a registry, and never carries a name or address.

## Tier 2 — officially reserved or explicitly fictional values

Some authorities publish identifiers that are reserved precisely so nobody is
identified by them:

- **Mexico — SAT generic RFCs** `XAXX010101000` (national) and `XEXX010101000`
  (foreign), reserved by RMF 2026 rule 2.7.1.23 (link in
  `crates/countries/mex/README.md`). They are the "public in general" taxpayer;
  `XAXX010101000` does not close its check digit, which is why the crate accepts
  both before the checksum.
- **Worked examples printed in an official algorithm manual**, when the manual
  presents them as an example (for example the alphanumeric CNPJ in the Receita
  Federal / Serpro DV manual). Re-derive the check digit before using one, and
  cite the manual, not "the internet".

Tier 2 requires the source's own words: `ficticio`, `de prueba`, `ejemplo`,
`genérico`, `reservado`. A real number printed in an official PDF is still a real
number.

## Tier 3 — fake-data generators (acceptable, with a recorded seed)

These produce values that were never read from a person:

- **Python Faker** — https://faker.readthedocs.io — locale providers include
  `es_MX` (`curp()`), `es_CL`, `es_CO`, `pt_BR` (`cpf()`, `cnpj()`). Seed it
  (`Faker.seed(0)`) so the exact value is reproducible, and paste the seed into
  the test comment.
- **Faker.js** — https://fakerjs.dev — and **Bogus** (.NET) for the same purpose.
- **fake-rs** — https://docs.rs/fake — only as an out-of-repo generator; the
  workspace has no dev-dependencies.

Generator output is not reserved: treat it as synthetic provenance, and still
recompute the check digit with this repository's algorithm before asserting it.
Do not add a dependency to a validator crate to generate a fixture.

Do not use a site that both generates and *resolves* numbers (a "consulta" box,
a name, an address). That is a data broker, not a generator.

## Never acceptable

- People-search and consulta sites for cédula, RUT, CURP, NIT, DNI, CPF, RIF,
  DIMEX or RTN; public registries and taxpayer lookups.
- Leaked or scraped datasets, Pastebin, social media, screenshots, support
  tickets, chat logs.
- Tutorials, Stack Overflow, gists, blog posts, and library test suites
  (including python-stdnum's documented examples) used as fixtures.
- Any value that arrives with a name, birth date, address or photo.

## Acceptance checklist

- [ ] Provenance is recorded in the README and in the test comment.
- [ ] The value was derived here (Tier 1), is officially reserved (Tier 2), or
      was generated with a recorded seed (Tier 3).
- [ ] The check digit was recomputed here, not trusted from the source.
- [ ] No real name, address or birth date is attached.
- [ ] The value was never sent to an external lookup service.
- [ ] The README's synthetic section and the test agree.

## If a real number is already in the repository

1. Replace it with a Tier 1 equivalent of the same shape and branch.
2. Do not repeat the original in the commit message, the issue, the CHANGELOG or
   the report; say "removed a real identifier from the fixtures".
3. If the value was published (crates.io, release tag, public issue), tell the
   maintainer — it may need history rewriting, and that is their decision.
4. If a bug report contains one, reproduce with a synthetic value and ask the
   reporter for a masked equivalent when the exact value matters.
