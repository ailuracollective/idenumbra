# idenumbra-bra

Brazil identification validator.

## Documents

- CPF (11 digits, 2 check digits).
- CNPJ (14 characters, 2 check digits, alphanumeric root and branch since
  July 2026).

## Validation algorithm

CPF: 11 digits, repeated numbers rejected, weights 10..2 then 11..2, check
digits with `11 - (sum mod 11)` and `>= 10 -> 0`.

CNPJ: 14 characters — positions 1-12 (root and branch) may be digits or the 26
uppercase letters `A`-`Z`, positions 13-14 are numeric check digits. Each
character is valued as its ASCII code minus 48, so digits keep 0-9 and
`A`-`Z` map to 17-42; weights are `[5,4,3,2,9,8,7,6,5,4,3,2]` and
`[6,5,4,3,2,9,8,7,6,5,4,3,2]`, with `11 - (sum mod 11)` and `< 2 -> 0`
(IN RFB 2.229/2024 and the Receita Federal *Manual de Cálculo do DV*). An
all-repeated number is rejected as a placeholder, as in the Receita Federal
example code.

Dots, slashes, hyphens and spaces are ignored; lowercase input is uppercased.
Accented letters and `Ñ` are rejected: the official alphabet is ASCII `A`-`Z`
only. The ENCAT joint technical note (NT 2025.001) mentions a request to also
exclude `I`, `O`, `U`, `Q` and `F`; the note itself says that needs
confirmation from the Receita Federal, so this crate does not exclude them.

### Editable constants

None: this validator has no catalogue-derived constants; every check is algorithmic or structural.

## References

Official sources are listed first; the `implementation` ones are the cross-checks used where the agency does not publish the check-digit algorithm.

- **Official** — Receita Federal, *CNPJ Alfanumérico* (format `AA.AAA.AAA/AAAA-DV`,
  `A` = 0-9 or A-Z, from July 2026):
  https://www.gov.br/receitafederal/pt-br/acesso-a-informacao/acoes-e-programas/programas-e-atividades/cnpj-alfanumerico
  — checked 2026-11-17
- **Official** — Receita Federal / Serpro, *Cálculo dos dígitos verificadores de
  CNPJ alfanumérico* (weights, ASCII-48 table, worked example
  `12.ABC.345/01DE-35`):
  https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/documentos-tecnicos/cnpj/manual-dv-cnpj.pdf
  — checked 2026-11-17
- **Official** — Receita Federal, IN RFB nº 2.229/2024 (DOU 16/10/2024, section 1,
  p. 84; rectified DOU 25/10/2025): https://normasinternet2.receita.fazenda.gov.br/#/consulta/externa/141102
  — checked 2026-11-17
- **Official** — Receita Federal, news item on IN RFB 2.229/2024 (A=17, B=18, ...):
  https://www.gov.br/receitafederal/pt-br/assuntos/noticias/2024/outubro/cnpj-tera-letras-e-numeros-a-partir-de-julho-de-2026
  — checked 2026-11-17
- **Official** — Receita Federal, CNPJ alfanumérico questions and answers:
  https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf
  — checked 2026-11-17
- **Official** — Receita Federal, first alphanumeric CNPJ issued on 2026-07-31:
  https://www.gov.br/receitafederal/pt-br/assuntos/noticias/2026/julho/receita-federal-gera-o-primeiro-cnpj-em-formato-alfanumerico
  — checked 2026-11-17
- **Implementation** — ENCAT, *Nota Técnica Conjunta 2025.001* (regexp
  `[A-Z0-9]{12}[0-9]{2}`, unconfirmed letter exclusions):
  http://www.cte.fazenda.gov.br/portal/exibirArquivo.aspx?conteudo=cxpT6%2F6wST8%3D
  — checked 2026-11-17
- **Implementation** — https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/br/cnpj.py
  — checked 2026-11-17

The CPF and CNPJ check digits are published by the issuing agency; the
implementation links are only cross-checks.

## Usage

All features:
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra-std = { version = "0.1.0-alpha.1", features = ["bra"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("BRA", "12345678")?;
```

## Test numbers (synthetic, not real)

- CPF: `123.456.789-09`
- CNPJ: `12.345.678/0001-95`
- Alphanumeric CNPJ: `12.ABC.345/01DE-35` (the worked example printed in the
  Receita Federal / Serpro DV manual)
- Alphanumeric CNPJ: `ZZZZZZZZZZZZ62` and `A1B2C3D4E5F668` (locally generated)

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally; the alphanumeric vectors follow the Receita Federal *Manual de Cálculo do DV* (IN RFB 2.229/2024).

## Notes

- Feature flag: `bra`.
- Country code (ISO 3166-1 alpha-3): `BRA`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
