# idenumbra-cri

Costa Rica identification validator.

## Documents

- Cédula de identidad física (9 digits).
- Cédula jurídica (10 characters, alphanumeric consecutive in the second half
  of 2026; the Registro Público announces the exact date).
- NITE (10 digits, explicit `kind`; it shares the 10-character shape with the
  cédula jurídica).
- DIMEX (11-12 digits).
- Passport (alphanumeric).

## Validation algorithm

Length, alphabet and leading-zero based:

- cédula física: exactly 9 digits, no leading zero;
- cédula jurídica: exactly 10 characters, no leading zero, first 4 digits (1 for
  the type of legal person + 3 for its class) and the last 6 digits or uppercase
  letters (the consecutive);
- NITE: exactly 10 digits, no leading-zero rule stated, so `0123456789` is
  accepted; inference cannot tell a NITE from a cédula jurídica, so pass the
  kind when the difference matters;
- DIMEX: 11 or 12 digits, no leading zero;
- passport: 6-12 alphanumerics.

Costa Rican cédulas and the DIMEX carry no public check digit, so no arithmetic
check is applied. Neither does the cédula jurídica or the NITE.

The 12-digit DIMEX with a leading zero is rejected. The zero-padded form exists
only inside the 50-character invoice key (*nota 4.1*: "se debe agregar un cero al
inicio con el fin de completar 12 dígitos"), not as a document number.

The NITE follows the same rule. Hacienda's *Anexos y Estructuras* v4.4 says "El
Documento de Identificación de la DGT (NITE) debe contener 10 dígitos y sin
guiones", and *nota 4.1* pads it with two zeros to fill the 12-digit
identification field of the invoice key. The only document that asks for 12
characters is the municipal reporting resolution of 2025-02-06, whose annex
sizes the field as "NITE 12 caracteres (10 y 2 verificadores)" and gives no
algorithm for those two characters. The 12-character value is therefore a
reporting representation, not a canonical identifier, and `kind = "nite"`
rejects it. A 12-digit NITE and a 12-digit DIMEX overlap in length, which is why
the electronic invoice carries a separate type code; the number alone cannot be
classified.

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
  11 or 12 digits without leading zeros, NITE = "10 dígitos y sin guiones";
  *nota 4.1* pads the DIMEX and the NITE to 12 only to build the 50-character
  key: https://www.hacienda.go.cr/docs/ANEXOS_Y_ESTRUCTURAS_V4.4.pdf — checked
  2026-11-17
- **Official** — Ministerio de Hacienda, resolución para el suministro de
  información por parte de las Municipalidades (2025-02-06), annex: "Identificación
  DIMEX extranjero 12 caracteres. Identificación NITE 12 caracteres (10 y 2
  verificadores). Cédula jurídica 10 caracteres". The 12-character NITE is the
  reporting field, not the canonical number; the annex gives no algorithm for
  the two "verificadores", so no character is checked:
  https://www.hacienda.go.cr/docs/ResolucionSuministroInformacionMunicipalidades06feb2025.pdf
  — checked 2026-11-17
- **Official** — Ministerio de Hacienda, *Disposiciones técnicas de comprobantes
  electrónicos* (DGT-R-000-2024), *nota 4.1*: three zeros for the cédula física,
  two for the cédula jurídica, one for an 11-digit DIMEX, two for the NITE —
  each to complete the 12-digit identification field:
  https://www.hacienda.go.cr/docs/DGT-R-000-2024DisposicionesTecnicasDeComprobantesElectronicosCP.pdf
  — checked 2026-11-17
- **Official** — Ministerio de Hacienda, circular MH-DGA-CIR-0005-2026
  (2026-02-26) forwarding the alphanumeric cédula jurídica: the Registro Público
  will apply it "en el segundo semestre del año 2026, en la fecha que
  oportunamente será especificada", when the six-digit consecutive reaches
  999999; the consecutive keeps the same character count and only new legal
  persons get it (arts. 449 Código Civil and 137 of Decreto Ejecutivo
  44648-MJP): https://www.hacienda.go.cr/docs/MH-DGA-CIR-0005-2026.pdf —
  checked 2026-11-17
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
- **Implementation** — python-stdnum `cr.cpj` (cédula jurídica: validates the
  class/type catalog and requires digits) and `cr.cr` (DIMEX: requires first
  digit `1`): https://github.com/arthurdejong/python-stdnum/tree/master/stdnum/cr.
  Neither computes a check digit and there is no NITE module — no public package
  found implements a NITE check character. The class catalog is not applied here
  because the Registro Nacional revises it and the alphanumeric consecutive
  changes the field in 2026.

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
- NITE: `1234567890` and, with the leading zero the annex does not forbid,
  `0123456789`

These values are **synthetic** and do **not** correspond to any real person or company. Generated locally matching the validator format rules. Source: Ministerio de Hacienda *Anexos y Estructuras* v4.4 and the Registro Nacional communiqué on the alphanumeric cédula jurídica; the `3-101-A00001` shape follows the Registro Nacional's published example.

## Notes

- Feature flag: `cri`.
- Country code (ISO 3166-1 alpha-3): `CRI`.
- Lazy loading: only compiled when the corresponding Cargo feature is enabled.
- `validate_with("id_card" | "cedula_juridica" | "nite" | "dimex" | "passport")`
  forces the document type. `validate` infers by length: 9 digits are a cédula
  física, 10 characters a cédula jurídica or a NITE, 11-12 digits a DIMEX.
- What is not validated: whether the number exists or is active, the DIMEX
  holder's status, and whether a 10-digit value is a NITE or a cédula jurídica
  (use `kind`). The 2025 municipal resolution's "12 caracteres (10 y 2
  verificadores)" wording is unresolved: no Hacienda document defines a NITE
  check character, so none is checked and a 12-character NITE is rejected.
