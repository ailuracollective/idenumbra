# Abstractable components of Latin American identification numbers

Research date: 2026-11-18. Documentation only — no Rust code changes.

This document records, per country and document, which fields the number encodes
with **published, stable semantics** (region/province, taxpayer or person type,
municipality, embedded identifier, birth date, century, sex, serial, check
digit, …). A field is a *component* only when the issuing agency publishes its
meaning and allowed range; opaque serial counters are not components, even when
their length is fixed. Each field is graded like an algorithm (High / Medium /
Low / None) and must be verified against an official source before it is exposed
as a typed field in a crate.

Ecuador (`crates/countries/ecu`) is the reference implementation: its
`components(value)` exposes `province`, `person_type`/`taxpayer`, `serial`,
`check_digit`, an embedded `id_card` and a public-entity `serial`, and
deliberately omits private-company sequentials as ambiguous.

## Method

1. Read each crate's README and validators for current claims.
2. Identify the issuing authority.
3. Investigate official sources (issuer's own domain preferred: law →
   reglamento/decreto → agency resolution/circular → technical annex).
4. Grade confidence per field. A component backed only by a third-party
   implementation is Low/None and must not be exposed as a typed field.
5. Note which fields are catalogue data (department tables, issuer prefixes) —
   those become named `const`s, not free text.

## Summary (components only)

| ISO | Document | Abstractable components |
|---|---|---|
| ARG | DNI | none (opaque serial) |
| ARG | CUIT/CUIL | taxpayer-type prefix, embedded DNI (natural persons), check digit |
| BOL | CI | none (opaque serial; complement is duplicate-resolution code) |
| BOL | NIT | embedded CI, taxpayer/person-type control code (01/02/03/04) |
| BRA | CPF | fiscal-region digit (9th), check digits (×2) |
| BRA | CNPJ | legal-entity root (8), establishment-order/branch (4), check digits (×2) |
| CHL | RUT/RUN | none (opaque serial) + check digit |
| COL | CC | none (opaque serial, NUIP) |
| COL | NIT | check digit; embedded cédula (natural persons) |
| CRI | cédula física | birth province (1st digit) |
| CRI | cédula jurídica | person type (1st digit), class (3 digits) |
| CRI | NITE / DIMEX | none (NITE); country code (DIMEX) |
| CUB | carné | birth date (YYMMDD), century code (7th digit) |
| DOM | cédula | check digit (Luhn) only |
| ECU | cédula | province, person type, serial, check digit |
| ECU | RUC | province, taxpayer, embedded id_card (natural), serial (public) |
| GTM | CUI/DPI | check digit, department, municipality |
| GTM | NIT | check digit |
| HND | DNI | none (opaque serial) |
| HND | RTN | embedded DNI (natural persons) |
| MEX | CURP | name code, birth date, sex, state code, internal consonants, century differentiator, check digit |
| MEX | RFC | name letters, birth date, check character |
| NIC | cédula | birth date; municipality code (position published, table not) |
| PAN | cédula | province code, marker (E/N/PE/AV/PI) |
| PAN | RUC | province code, NT tag, folio, asiento |
| PER | DNI | none (opaque serial) |
| PER | RUC | taxpayer-type prefix, embedded DNI (prefix 10), check digit |
| PRY | CI | none (opaque serial) |
| PRY | RUC | check digit |
| SLV | DUI | check digit only |
| SLV | NIT | taxpayer type (1st digit), municipality of birth, birth date, sequential, check digit |
| URY | CI | check digit only |
| URY | RUT | operating-unit code, sequential taxpayer number |
| VEN | CI | none (opaque serial) |
| VEN | RIF | taxpayer/person-type letter, embedded identifier, check digit |

---

## ARG — Argentina

### DNI (`dni`)
- Components: none (opaque serial only)
- Source: RENAPER, Disposición 902/2021, layout of the DNI card (https://www.argentina.gob.ar/normativa/nacional/norma-381658/texto) — checked 2026-11-18
- Confidence: High
- Notes: 7-8 digits, no check digit. RENAPER publishes no region/sex/department mapping for the numeric DNI. The F/M prefix printed before matrículas < 10.000.000 is a card-display convention, not part of the validated number. All-zero rejected as placeholder.

### CUIT/CUIL (`cuit`/`cuil`)
- Components: taxpayer-type prefix (2 digits), embedded DNI (8 digits, natural persons), check digit (1)
- Source: RG Conjunta AFIP-ANSES 5007/2021, art. 1 (https://www.argentina.gob.ar/normativa/nacional/resoluci%C3%B3n-5007-2021-350854) and ARCA Constancia de Inscripción record layout (https://www.arca.gob.ar/genericos/cInscripcion/archivoCompleto.asp) — checked 2026-11-18
- Confidence: Medium
- Notes: Natural-person CUIT = prefix + DNI + DV is stated by AFIP/ARCA. Only 20/23/24/27 for natural persons appears in a norm (RG 5007/2021) and those are assigned at random since 2022; 30/33/34 (legal persons) and 50/51/55 (foreign) come from ARCA service documentation, with no normative prefix table. Check digit is modulo 11 but ARCA publishes no algorithm (crate cross-checks). Prefix is deliberately not enforced.

## BOL — Bolivia

### CI (`id_card`)
- Components: none in the number (opaque serial only); optional complement is a duplicate-resolution code assigned by SEGIP
- Source: SEGIP Reglamento del Registro Único de Identificación Personal, art. 40 (https://www.segip.gob.bo/fuent/2025/09/REGLAMENTO-REGISTRO-UNICO-IDENTIFICACION-PERSONAL.pdf) — checked 2026-11-18
- Confidence: High
- Notes: The CI number itself is a sequential registry number with no published components. The complement (1-2 alphanumerics after `-`/space) is a published duplicate-resolution code. The department extension (LP, SC, …) is a separate printed field, not part of the number, and is not validated. No check digit exists.

### NIT (`nit`)
- Components: embedded CI (natural persons / unipersonals), taxpayer/person-type control code (01/02/03/04), opaque random digit; no check digit
- Source: SIN/SIAT, "Generación del NIT" (https://siatanexo.impuestos.gob.bo/index.php/requisitos-para-la-inscripcion/conceptos-generales/generacion-del-nit) — checked 2026-11-18
- Confidence: High
- Notes: SIN publishes NIT = CI + code `01` + random digit (natural person/unipersonal), code `02` for legal persons, code `04` for foreigners. The control code is a person-type marker, not a check digit; SIAT's "Módulo 11" computes the CUF (invoice code), not the NIT. Length 9-11 and the `03` code are observed from SIN supplier lists, not published (Medium for those).

## BRA — Brazil

### CPF (`cpf`)
- Components: fiscal-region digit (9th digit, 0-9 mapping to 10 Receita Federal fiscal regions), check digits (2)
- Source: Receita Federal, "Cadastro de Pessoas Físicas" folheto (https://www.gov.br/receitafederal/pt-br/assuntos/educacao-fiscal/educacao-fiscal/publicacoes/folhetos/cpf.pdf) — checked 2026-11-18
- Confidence: High
- Notes: The 9th digit encodes the fiscal region of the address at first registration (Receita Federal table: 0=RS, 1=DF/GO/MT/MS/TO, …, 8=SP, 9=PR/SC). The first 8 digits are an opaque serial assigned at registration. Receita Federal notes exceptions may exist. Check digit algorithm is officially published (modulo 11).

### CNPJ (`cnpj`)
- Components: legal-entity root (8 chars, shared across establishments), establishment-order/branch code (4 chars, `0001` = headquarters at generation), check digits (2)
- Source: Receita Federal / Serpro, "Cálculo dos dígitos verificadores de CNPJ alfanumérico" + CNPJ alfanumérico Q&A (https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/documentos-tecnicos/cnpj/manual-dv-cnpj.pdf and https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf) — checked 2026-11-18
- Confidence: High
- Notes: Official layout splits 14 positions into 8 (root) + 4 (order) + 2 (DV). Root identifies the legal entity and is shared by matriz and filiais; `0001` indicates the headquarters at generation (Receita Federal states this is initial, not permanent). Since July 2026 root and branch may be alphanumeric (A-Z, ASCII-48 valuation). Root is otherwise an opaque serial.

## CHL — Chile

### RUT/RUN (`rut`/`run`)
- Components: none (opaque serial only) + check digit (1, 0-9 or K)
- Source: SII, "Inscripción y obtención de N° de RUT" and boleta electrónica format (https://www.sii.cl/servicios_online/1031-1032.html and https://www.sii.cl/factura_electronica/factura_mercado/formato_boleta_electronica.pdf) — checked 2026-11-18
- Confidence: High
- Notes: SII publishes only the format (7-8 digit body + check character) and the modulo-11 check; the body is a sequential identifier assigned by the Registro Civil/SII. No region/sex/date/establishment components are published. Weights are not published by SII (long-standing public specification). Generic RUT `66.666.666-6` passes but identifies no person.

## COL — Colombia

### Cédula de ciudadanía (`id_card`)
- Components: none (opaque serial only)
- Source: Registraduría Nacional, Resolución 3571 de 2003 (NUIP 10 numeric digits, from 1.000.000.000) and Circular Única v9 (https://www.registraduria.gov.co/IMG/pdf/circular_unica_v_9.pdf and https://cancilleria.gov.co/sites/default/files/Normograma/docs/pdf/resolucion_registraduria_3571_2003.pdf) — checked 2026-11-18
- Confidence: High
- Notes: NUIP is assigned by registration office; Registraduría explicitly states the current number is not derived from birth date/sex (unlike the historical NIP). Older 8-digit cédulas and 11-digit NIPs (minors) are legacy. No check digit. Note old NIP encoded birth date + sex + check digit, but that is the tarjeta de identidad, not the cédula.

### NIT (`nit`)
- Components: check digit (1); embedded cédula or 11-digit civil-registry number for natural persons
- Source: DIAN, Orden Administrativa 4/1989 annex 3 (weights) and Concepto 78424 de 2002 (https://normograma.dian.gov.co/dian/compilacion/docs/orden_administrativa_dian_0004_1989.htm and https://normograma.dian.gov.co/dian/compilacion/docs/concepto_tributario_dian_0078424_2002.htm) — checked 2026-11-18
- Confidence: High
- Notes: DIAN publishes that a natural person's NIT is their cédula (or 11-digit civil registry number for minors) plus the DIAN check digit; legal persons get a DIAN-assigned body. Check digit is modulo 11 with DIAN weights (41…3 for 9-digit bodies; 43, 47 extension for 10-11 digit natural-person bodies is implementation practice, not in the annex). No other components.

### Passport (`passport`)
- Components: none (opaque serial only)
- Source: Cancillería / Registraduría identification definitions (https://www.registraduria.gov.co/Definiciones-de-identificacion.html) — checked 2026-11-18
- Confidence: None
- Notes: Alphanumeric, 6-12 chars. No published structure or check digit found in official sources; treat as opaque serial.

## CRI — Costa Rica

### Cédula de identidad física (`id_card`)
- Components: birth province (first digit `1`–`9`: `8` naturalized, `9` special birth registration)
- Source: TSE/Registro Civil (cited via README and Wikipedia summary of the TSE 2010 document) — checked 2026-11-18
- Confidence: Medium
- Notes: The remaining 8 digits are `tomo` (4) + `asiento` (4) registration references; their positions are published but no allowed ranges are, so they are treated as an opaque serial and not exposed as components. No check digit is published.

### Cédula jurídica (`cedula_juridica`)
- Components: person type (first digit, `2`–`5` by legal-person class); class (3-digit code from the Registro Nacional published list)
- Source: Registro Nacional class list; cross-check python-stdnum `cr.cpj` — checked 2026-11-18
- Confidence: Medium
- Notes: The trailing 6 characters are the consecutive (opaque serial; not a component). The class catalog is revised by the Registro Nacional, so the crate does not enforce it.

### NITE (`nite`)
- Components: none (opaque 10-digit identifier)
- Source: Hacienda, "Anexos y Estructuras" v4.4 (https://www.hacienda.go.cr/docs/ANEXOS_Y_ESTRUCTURAS_V4.4.pdf) — checked 2026-11-18
- Confidence: High
- Notes: Hacienda publishes only length (10 digits, no leading-zero rule); no embedded fields.

### DIMEX (`dimex`)
- Components: country code (3 digits, published by the DGME)
- Source: DGME (https://migracion.go.cr/dimex/); cross-check python-stdnum `cr.cr` — checked 2026-11-18
- Confidence: Medium
- Notes: First digit is fixed to `1`; the remainder are serial counters (opaque, not components). No check digit.

### Passport (`passport`)
- Components: none
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: High

## CUB — Cuba

### Carné de identidad (`id_card`)
- Components: birth date (YYMMDD, validated as a real calendar date); century code (7th digit: `9` = 19th, `0`–`5` = 20th, `6`–`8` = 21st)
- Source: secondary documentation (Cubadebate matematización del carné) — checked 2026-11-18
- Confidence: Low
- Notes: No Cuban agency publishes the carné layout (Gaceta Oficial MININT Resolución 14/2014 lists the number among card data only; Ley 188/2026 publishes no format). The sex digit (10th, even/odd) is an unpublished convention and is not validated; the 2-digit serial and the control digit are unpublished/opaque. Century ranges come from secondary documentation.

### Passport (`passport`)
- Components: none
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: High

## DOM — Dominican Republic

### Cédula (`id_card`)
- Components: check digit (Luhn mod 10 over 11 digits)
- Source: cross-check (DGII API blog); issuer JCE/DGII publishes no table — checked 2026-11-18
- Confidence: Medium
- Notes: The first 3 digits encode municipality/registry and the sequence encodes sex, but the JCE/DGII publish no table or allowed range, so those fields are not exposed as components. Luhn is not officially published by the issuer (reconstructed from cross-checks).

### Passport (`passport`)
- Components: none
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: High

## ECU — Ecuador (reference confirmation)

### Cédula (`id_card`)
- Components: province (INEC code `01`–`24`, or `30` registered abroad, with name); person type (third digit, `0`–`5`, no finer meaning published); serial (digits 4–9); check digit (mod 10, weights `[2,1,2,1,2,1,2,1,2]`)
- Source: Registro Civil, *Preguntas frecuentes – Cedulación* (https://www.registrocivil.gob.ec/preguntas-frecuentes-cedulacion/); INEC División Política Administrativa (https://www.ecuadorencifras.gob.ec/documentos/web-inec/Geografia_Estadistica/Micrositio_geoportal/index.html) — checked 2026-11-18
- Confidence: High

### RUC (`ruc`)
- Components: province (INEC code, plus legacy `50`); taxpayer (`NaturalPerson` for `0`–`5`, `PublicEntity` for `6`, `PrivateCompany` for `9`); `id_card` (embedded cédula for natural persons); `serial` (5 digits for public entities)
- Source: SRI, Reglamento a la Ley del RUC (Decreto 2167, updated 2024-02-09), art. 3 (https://www.sri.gob.ec/o/sri-portlet-biblioteca-alfresco-internet/descargar/cc9751bf-9123-4219-bf2c-79daba57116a/Reglamento_Ley_RUC_ultmodificacion_09022024.pdf); OECD, *Information on Tax Identification Numbers – Ecuador* (https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/ecuador-tin.pdf) — checked 2026-11-18
- Confidence: High
- Notes: Private-company sequentials are not exposed (variable length, cannot be split from the check digit). The mod 11 digit for public entities is validated; private-company RUCs get a structural check only.

### Passport (`passport`)
- Components: none (`Ok(None)`)
- Source: structure only, 6–9 alphanumerics — checked 2026-11-18
- Confidence: High

## GTM — Guatemala

### CUI/DPI (`cui`/`dpi`)
- Components: check digit (SAT mod 11 over the 8-digit correlativo, weights 2–9); department (`01`–`22`); municipality (per department, INE/SAT table, 340 total)
- Source: SAT FEL, "Reglas y validaciones" 2.1, §5.1.4–5.1.5 (https://portal.sat.gob.gt/portal/descarga/14852/reglas-y-validaciones/97423/fel_reglas_y_validaciones-3); RENAP (https://www.renap.gob.gt/noticias/que-es-el-cui); INE departmental presentation (https://www.ine.gob.gt/ine/wp-content/uploads/2020/08/02-Presentacion-de-Resultados-Departamentales-y-Municipales.pdf) — checked 2026-11-18
- Confidence: High
- Notes: The 8-digit `correlativo` is an opaque serial assigned by RENAP and is not a component. The 12-digit form is the same layout with a dropped leading zero.

### NIT (`nit`)
- Components: check digit (SAT mod 11, `10` → `K`)
- Source: SAT FEL, "Reglas y validaciones" 2.1, §5.1.3 (https://portal.sat.gob.gt/portal/descarga/14852/reglas-y-validaciones/97423/fel_reglas_y_validaciones-3); SAT FAQ on taxpayers (https://portal.sat.gob.gt/portal/preguntas-frecuentes/informacion-de-contribuyentes/) — checked 2026-11-18
- Confidence: High
- Notes: The body is an opaque taxpayer number (or a CUI prefix for natural persons); no embedded region/type field is published. A 13-digit value is validated as a CUI.

### Passport (`passport`)
- Components: none
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: High

## HND — Honduras

### DNI (`dni`)
- Components: none (opaque serial; RNP publishes neither layout nor algorithm)
- Source: RNP (https://www.rnp.hn, cited via README) — checked 2026-11-18
- Confidence: None
- Notes: Issuer RNP. The 4+4+5 `municipio/año/correlativo` split is visible in public data but is not an official rule; not validated.

### RTN (`rtn`)
- Components: embedded DNI (natural-person RTN = DNI + 1 digit, per SAR); extra digit opaque (no published meaning)
- Source: SAR (https://www.sar.gob.hn/registro-tributario-nacional-rtn) — checked 2026-11-18
- Confidence: Medium
- Notes: Issuer SAR. 14 digits since 2003; four taxpayer categories (natural without obligations, natural with obligations, individual merchant, legal entity) are registration classes, not encoded in the number. No published check-digit algorithm; SAR "Verificación de RTN" is the oracle.

### Passport (`passport`)
- Components: none (opaque)
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: None

## MEX — Mexico

### CURP (`curp`)
- Components: name code (4 letters), birth date (YYMMDD), sex (H/M/X), state code (2, published Anexo I table), internal consonants (3), century differentiator (position 17: `0`–`9` up to 1999 / `A`–`J` from 2000), check digit (18)
- Source: RENAPO/SEGOB Instructivo Normativo para la Asignación de la CURP (https://www.gob.mx/cms/uploads/attachment/file/337251/Instructivo_Normativo_para_la_Asignacion_de_la_CURP.pdf) — checked 2026-11-18
- Confidence: High
- Notes: Issuer RENAPO/SEGOB (DOF 1996 Acuerdo; Instructivo Normativo 2021). Homoclave position doubles as century indicator; inconvenient-word catalogue governs generation only.

### RFC (`rfc`)
- Components: name letters (3 companies / 4 persons), birth date (YYMMDD), check character (last position; `10` written as `A`)
- Source: SAT (https://www.sat.gob.mx/cs/Satellite?blobcol=urldata&blobkey=id&blobtable=MungoBlobs&blobwhere=1461174992692&ssbinary=true) — checked 2026-11-18
- Confidence: High
- Notes: Issuer SAT. Century not encoded (dates read as 20xx). Homoclave (3 trailing alphanumerics) is SAT-assigned opaque disambiguator. Generic RFCs `XAXX010101000` / `XEXX010101000` published (RMF 2026 rule 2.7.1.23). `Ñ` and `&` accepted in name part.

## NIC — Nicaragua

### Cédula de identidad (`id_card`)
- Components: birth date (DDMMYY); municipality code (3 digits) position defined by Ley 152 art. 31 but CSE publishes no code table (not verifiable)
- Source: Ley 152 (http://legislacion.asamblea.gob.ni/normaweb.nsf/b92aaea87dac762406257265005d21f7/64f20479f029610506258ab60059454d?OpenDocument) — checked 2026-11-18
- Confidence: Medium
- Notes: Issuer CSE. Serial (4 progressive digits) is opaque counter. Check letter uses community-derived mod-23 rule (CSE publishes no algorithm). Born-abroad code `888` from single third-party source only.

### Passport (`passport`)
- Components: none (opaque)
- Source: CSE (https://www.cse.gob.ni/cedulacion) — checked 2026-11-18
- Confidence: None
- Notes: 6–12 alphanumerics, structure only.

## PAN — Panama

### Cédula (`id_card`)
- Components: province code (01–13), marker (E extranjero, N naturalizado, PE panameño nacido en el exterior, AV antes de vigencia, PI población indígena)
- Source: Tribunal Electoral (archived, https://web.archive.org/web/20100728085351/http:/www.tribunal-electoral.gob.pa/cedulacion/index.html) — checked 2026-11-18
- Confidence: High
- Notes: Issuer Tribunal Electoral (Decreto 53/1958 art. 9). Tomo (rollo) and asiento (entry) are registry references / opaque serials. No published check digit.

### RUC (`ruc`)
- Components: province code (01–13), NT tag (DGI tax-number tag), folio and asiento (opaque serials); plus cédula shape for natural persons
- Source: DGI (https://dgi.mef.gob.pa/_7FacturaElectronica/source/4-Anexo-3-Ficha-T%C3%A9cnica-Factura-Electr%C3%B3nica-Proveedores-Autorizaci%C3%B3n-Calificados-V1.0.pdf) — checked 2026-11-18
- Confidence: Medium
- Notes: Issuer DGI. Two-digit DV is type-dependent (natural/jurídica/natural NT/jurídica NT/finca) and changes on registration; not derivable from the number alone. Use DGI eTax 2.0 DV service.

### Passport (`passport`)
- Components: none (opaque)
- Source: Tribunal Electoral (https://www.tribunal-electoral.gob.pa/direccion-nacional-de-cedulacion/la-cedula) — checked 2026-11-18
- Confidence: None
- Notes: 6–12 alphanumerics, structure only.

## PER — Peru

### DNI (`dni`)
- Components: none (opaque; RENIEC publishes no fields or check digit)
- Source: RENIEC (https://identidad.reniec.gob.pe/preguntas-frecuentes-dni) — checked 2026-11-18
- Confidence: None
- Notes: Issuer RENIEC. 8 digits.

### RUC (`ruc`)
- Components: taxpayer-type prefix (10 natural person with DNI, 15 foreign natural person, 20 legal person — published; 17 implementation-only), embedded DNI (prefix 10), check digit (last position)
- Source: OECD, *Information on Tax Identification Numbers – Peru* (https://www.oecd.org/content/dam/oecd/en/topics/policy-issue-focus/aeoi/peru-tin.pdf) — checked 2026-11-18
- Confidence: High
- Notes: Issuer SUNAT. Random number (8/9 digits) is opaque. Check-digit algorithm published via SUNAT's own form validator (`validRUC` in `normalization.js`), not in legislation. Prefix `17` accepted on implementation evidence only.

### Carné de extranjería / passport (`passport`/`ce`)
- Components: none (opaque)
- Source: SUNAT (https://www.sunat.gob.pe/legislacion/ruc/) — checked 2026-11-18
- Confidence: None
- Notes: 6–12 alphanumerics, structure only. Issuer Migraciones/SUNAT.

## PRY — Paraguay

### CI (`ci`)
- Components: none — opaque serial only (correlative from 1)
- Source: Policía Nacional (https://www.policianacional.gov.py/identificaciones/quienes-somos/) — checked 2026-11-18
- Confidence: High
- Notes: Policía Nacional publishes no length or check digit; the 6–8 digit range is observed practice.

### RUC (`ruc`)
- Components: check digit (mod 11, weights 2..11 from the right; remainders 0/1 → 0); body is an opaque identifier (CI for individuals; 80,000,000 series for companies; 50,000,000 for refugee applicants, per DNIT records)
- Source: DNIT, "Dígito Verificador" (https://www.dnit.gov.py/documents/20123/224893/D%C3%ADgito+Verificador.pdf/fb9f86c8-245d-9dad-2dc1-ac3b3dc307a7) and lookuptax cross-check (https://www.lookuptax.com/validate/paraguay/ruc) — checked 2026-11-18
- Confidence: Medium (check digit High; series meaning observed in DNIT records, not an explicit published rule)
- Notes: SIFEN allows a 3–8 character alphanumeric body; the legacy letter (A–D) form is not supported.

### Passport (`passport`)
- Components: none / opaque serial only
- Source: no published structure — checked 2026-11-18
- Confidence: None
- Notes: Only length (6–12) and alphanumeric charset are checked.

## SLV — El Salvador

### DUI (`dui`)
- Components: check digit (RNPN confirms the last digit is a verifier; mod-10 weights 9..2 are reverse-engineered); serial is a sequential counter
- Source: press summary of RNPN (https://www.eleconomista.net/tendencias/Los-nueve-numeros-del-DUI-que-significan-por-que-ninguno-empieza-con-el-uno-y-cual-es-la-importancia-del-digito-final-20241201-0004.html) — checked 2026-11-18
- Confidence: Medium (role High; weights not officially published)
- Notes: No municipality/region code is encoded; the README's "leading zero encodes the municipality" is not supported by RNPN (assignment is sequential order of presentation).

### NIT (`nit`)
- Components: taxpayer type via first digit (0/1 nationals, 9 foreigners); municipality of birth code (4 digits); birth date DDMMYY (6 digits); sequential correlative (3 digits); check digit (mod 11, weights reverse-engineered)
- Source: MH/DGII (https://www.mh.gob.sv/servicios/termino-de-numero-de-identificacion-tributaria-nit-por-duplicidad/) and lookuptax (https://www.lookuptax.com/docs/es/numero-identificacion-fiscal/guia-nit-el-salvador) — checked 2026-11-18
- Confidence: Medium-High
- Notes: Check-digit weights and the ≤100 legacy-correlative cut-off are reverse-engineered; DGII publishes neither.

### Passport (`passport`)
- Components: none / opaque serial only
- Source: no published structure — checked 2026-11-18
- Confidence: None
- Notes: Only length (6–12) and alphanumeric charset are checked.

## URY — Uruguay

### CI (`id_card`)
- Components: check digit (Decreto 501/1978 fixes modulus 10 only; weights `[2,9,8,7,6,3,4]` come from implementations); body is a sequential number
- Source: Decreto 501/1978 (https://www.impo.com.uy/bases/decretos/501-1978) — checked 2026-11-18
- Confidence: Medium
- Notes: No region, sex, or birth-date fields are published.

### RUT (`rut`)
- Components: operating-unit code (2 digits, 1–22; 22 for all new RUTs since 2024-10); sequential taxpayer number (6 digits); check digit (algorithm classified reserved by Resolución 836/2024); 3-digit suffix is an observed constant (001) with no published meaning
- Source: DGI (https://www.gub.uy/direccion-general-impositiva/comunicacion/noticias/nueva-numeracion-del-rut and https://www.gub.uy/ministerio-economia-finanzas/institucional/informacion-gestion/informes-solicitudes-acceso-informacion-publica/respuesta-100) — checked 2026-11-18
- Confidence: Medium
- Notes: Check-digit weights and the suffix meaning come from implementations/observation, not from DGI.

### Passport (`passport`)
- Components: none / opaque serial only
- Source: no published structure — checked 2026-11-18
- Confidence: None
- Notes: Only length (6–12) and alphanumeric charset are checked.

## VEN — Venezuela

### CI (`id_card`)
- Components: none / opaque serial only (6–8 digits)
- Source: SENIAT (http://www.seniat.gob.ve/, returns 403 to automated clients) — checked 2026-11-18
- Confidence: High
- Notes: SENIAT publishes no structure or check digit for the cédula; all-zero placeholders are rejected by convention.

### RIF (`rif`)
- Components: taxpayer/person-type letter (V/E/J/P/G/C, with C = J = 12); embedded identifier (8 digits = CI for natural persons, taxpayer number for legal entities); check digit (mod 11, reverse-engineered)
- Source: Wikipedia summary (https://es.wikipedia.org/wiki/Registro_%C3%9Anico_de_Informaci%C3%B3n_Fiscal) — checked 2026-11-18
- Confidence: High for letter/embedded-ID semantics; Medium for check digit
- Notes: `C` was introduced in 2015 by migrating comunas already registered under `J`, so both keep base 12; SENIAT publishes neither the letter table nor the formula.

### Passport (`passport`)
- Components: none (opaque)
- Source: structure only, 6–12 alphanumerics — checked 2026-11-18
- Confidence: High

---

## Next steps

- Per country/document, verify each candidate component against an official source and grade it before exposing it as a typed field.
- Decide, per document, whether `components` returns `Some` or `None`, and which fields are omitted (opaque serials).
- Only then implement the `Components` structs per crate, following the ECU pattern, and wire them into `crates/std` (five places, see `crate-wiring.md`).
- Update each crate README's "Components" section together with the code.
