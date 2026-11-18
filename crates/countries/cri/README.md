# idenumbra-cri

Costa Rica identification validator.

## Documents

- Cédula de identidad física (9 digits).
- Cédula jurídica (10 characters, alphanumeric consecutive from late 2026).
- NITE (10 digits; shares the cédula jurídica shape).
- DIMEX (11-12 digits).
- Passport (alphanumeric).

## Validation algorithm

Length, alphabet and leading-zero based:

- cédula física: exactly 9 digits, no leading zero;
- cédula jurídica / NITE: exactly 10 characters, no leading zero, first 4
  digits (1 for the type of legal person + 3 for its class) and the last 6
  digits or uppercase letters (the consecutive);
- DIMEX: 11 or 12 digits, no leading zero;
- passport: 6-12 alphanumerics.

Costa Rican cédulas and the DIMEX carry no public check digit, so no
arithmetic check is applied. The cédula jurídica does not carry one either.

The 12-digit DIMEX with a leading zero is rejected. Hacienda's identification
field forbids a leading zero, and the zero-padded form exists only inside the
50-character invoice key (*nota 4.1*: "se debe agregar un cero al inicio con el
fin de completar 12 dígitos"), not as a document number. A NITE is 10 digits
and is accepted through the same shape as the cédula jurídica: the number alone
does not tell them apart.

Hyphens, dots and spaces are dropped (`1-0234-0567`, `3-101-123456`) and
lowercase letters are uppercased.

### Editable constants

- `ID_CARD_LEN = 9`, `JURIDICAL_LEN = 10`, `DIMEX_LENGTHS = [11, 12]`
  (`src/validators/validate.rs`) — accepted lengths. The DIMEX is 11 digits in
  the legacy format and 12 in the newer one; the tax authority accepts both,
  always without a leading zero. Drop `11` only if the legacy format is retired.
- `JURIDICAL_LEN = 10` must stay 10 when the consecutive turns alphanumeric:
  the Registro Nacional keeps the same character count (`3-101-A00001`).

## References

- **Official** — Ministerio de Hacienda, *Anexos y Estructuras* v4.4 (in force
  since 2025-09-01, log updated 2026-04-22): cédula física = 9 digits without a
  leading zero, cédula jurídica = 10 characters (alphanumeric allowed), DIMEX =
  11 or 12 digits without leading zeros, NITE = 10 digits; *nota 4.1* pads the
  DIMEX to 12 only to build the 50-character key:
  https://www.hacienda.go.cr/docs/ANEXOS_Y_ESTRUCTURAS_V4.4.pdf — checked 2026-11-17
- **Official** — Ministerio de Hacienda, circular MH-DGA-CIR-0005-2026
  (2026-02-26) forwarding the alphanumeric cédula jurídica:
  https://www.hacienda.go.cr/docs/MH-DGA-CIR-0005-2026.pdf — checked 2026-11-17
- **Official** — Registro Nacional, *Comunicado oficial: cambio de formato de
  la cédula jurídica* (6-character consecutive becomes alphanumeric in Q4 2026,
  same length; examples `3-101-A00001`):
  https://servicios.rnpdigital.com/personas_juridicas/documentos/Comunicado%20DG%20cambio%20formato%20de%20cedula%20juridica.pdf
  — checked 2026-11-17
- **Official** — Decreto Ejecutivo 44648-MJP (2024-09-05), art. 137,
  *Conformación de la cédula jurídica o número identificador*:
  https://pgrweb.go.cr/scij/Busqueda/Normativa/Normas/nrm_articulo.aspx?nValor1=1&nValor2=102851&nValor3=142498&nValor5=138&param1=NRA
  — checked 2026-11-17
- **Official** — Dirección General de Migración y Extranjería, DIMEX:
  https://migracion.go.cr/dimex/ — checked 2026-11-17
- **Official** — Tribunal Supremo de Elecciones, cédula de identidad:
  https://www.tse.go.cr/cedula.html — checked 2026-11-17
- **Official (international)** — OECD, *Costa Rica TIN*:
  https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/costa-rica-tin.pdf
  — checked 2026-11-17
- **Implementation** — https://es.wikipedia.org/wiki/C%C3%A9dula_de_identidad_(Costa_Rica)
  — checked 2026-11-17

No official source publishes a cédula, cédula jurídica or DIMEX check digit;
the TSE and the Registro Nacional issue and verify the numbers themselves, so
this crate validates structure only.

## Usage

All features:
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["all"] }
```

Single feature (only the country you need):
```toml
[dependencies]
idenumbra = { version = "0.1", features = ["cri"] }
```

```rust
use idenumbra::Idenumbra;
Idenumbra::infer("CRI", "12345678")?;
```

## Test numbers (synthetic, not real)

- Cédula física: `123456789`
- DIMEX: `12345678901` and `123456789012`
- Cédula jurídica: `3101123456` and `3101A00001`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally matching the validator format rules. Source: Ministerio de Hacienda *Anexos y Estructuras* v4.4 and the Registro Nacional communiqué on the alphanumeric cédula jurídica; the `3-101-A00001` shape follows the Registro Nacional's published example.

## Notes

- Feature flag: `cri`.
- Country code (ISO 3166-1 alpha-3): `CRI`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
