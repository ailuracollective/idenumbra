# idenumbra-bra

Brazil identification validator.

## Documents

- CPF (11 digits, 2 check digits).
- CNPJ (14 characters, 2 check digits, alphanumeric root since July 2026).

## Validation algorithm

CPF: 11 digits, repeated numbers rejected, weights 10..2 then 11..2, check
digits with `11 - (sum mod 11)` and `>= 10 -> 0`. CNPJ: 14 alphanumerics
(letters are allowed in the first 12 positions, check digits are numeric),
weights `[5,4,3,2,9,8,7,6,5,4,3,2]` and `[6,5,4,3,2,9,8,7,6,5,4,3,2]`; each
character is valued as its ASCII code minus 48 (IN RFB 2.229/2024). Dots,
slashes, hyphens and spaces are ignored; lowercase input is uppercased.

## References

- https://www.gov.br/receitafederal/pt-br/assuntos/orientacao-tributaria/cadastros/cnpj/cnpj-alfanumerico — checked 2026-11-17
- https://www.gov.br/receitafederal/pt-br/assuntos/orientacao-tributaria/cadastros/cpf — checked 2026-11-17
- https://github.com/arthurdejong/python-stdnum/blob/master/stdnum/br/cnpj.py — checked 2026-11-17

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["bra"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("BRA", "12345678")?;
```

## Test numbers (synthetic, not real)

- CPF: `123.456.789-09`
- CNPJ: `12.345.678/0001-95`
- Alphanumeric CNPJ: `12.ABC.345/01DE-35`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally with this crate's validation algorithm. Source: generated locally with the validator algorithm; alphanumeric CNPJ per the Receita Federal rules (IN RFB 2.229/2024).

## Notes

- Feature flag: `bra`.
- Country code (ISO 3166-1 alpha-3): `BRA`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
