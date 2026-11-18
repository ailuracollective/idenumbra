# Validation limits and where to verify

`Ok` means the value has the documented shape and, where the issuing agency
publishes one, a matching check digit. It does **not** mean the number exists,
is assigned, is active, or belongs to the person presenting it. Use the issuing
registry for that (table at the end).

This file is the cross-country index of the residual risks. Each crate README
has the per-document detail, the editable constants and the sources with their
`checked YYYY-MM-DD` date.

## Residual risks by country

| Country | Document | What remains uncertain | Effect | Mitigation |
|---|---|---|---|---|
| BOL | NIT | No public check digit (the SIAT "Módulo 11" page is the CUF). The SIAT page documents codes `01`/`02`/`04` and a random digit `1`-`9`; code `03` and a final `0` appear only in SIN supplier lists. Lengths 9-11 are observed, not published. | A future code or an 8-digit NIT is rejected by `kind = "nit"`; `validate` reads 8 digits as a CI. | Use `kind = "nit"`; verify in SIN. |
| COL | Cédula | No check digit; 6-10 digits are shape only. | A random 6-10 digit string is accepted. | Verify in Registraduría. |
| COL | NIT | The check digit is only checked with `kind = "nit"`; 10 digits without a kind are a cédula. Bodies of 12+ digits are rejected (the 15-prime table has no official source). | A NIT with a wrong digit passes `validate`. | Always pass `kind = "nit"` for tax IDs. |
| CRI | Cédula, DIMEX, cédula jurídica, NITE | No public check digit. The NITE is canonically 10 digits ("10 dígitos y sin guiones", Anexos v4.4) and shares the 10-character shape with the cédula jurídica; a 2025 municipal reporting resolution describes a 12-character NITE field ("10 y 2 verificadores") that no other Hacienda document defines. A DIMEX padded with a leading zero (the invoice-key form) is rejected. | Shape only; NITE and cédula jurídica are indistinguishable by shape, and a 12-character NITE (if one exists) is rejected. | Use `kind = "nite"` / `"cedula_juridica"`; verify in TSE / Registro Nacional. |
| CUB | Carné | The control digit algorithm and the layout are not published; the 7th digit is a century marker. The sex digit cannot be validated without knowing the sex. The 2026 law does not publish a new numbering. | Shape/date only. | Verify with MININT; no public online service. |
| DOM | Cédula | Luhn is the public rule, but about 576 real cédulas issued before the check digit do not satisfy it. They are not included because they are personal data. | Those cédulas are false negatives. | Ask for a synthetic equivalent if a user reports one; verify in DGII/JCE. |
| ECU | Cédula, RUC | The SRI publishes neither the check digit nor the third-digit table. Company RUCs with a long sequential are structure-only (the SRI stopped guaranteeing modulo 11). Province `50` is legacy without an official source. | Public-company RUCs still use modulo 11; a wrong digit can pass on private companies. | Verify in SRI. |
| GTM | CUI/DPI | If RENAP maps a check-digit remainder of `10` to `0`, the crate rejects those CUIs (about 1 in 11). SAT's 2.1 table still lists 32 municipalities for Huehuetenango while INE lists 33 (Petatán, `1333`). Naturalized people and domiciled foreigners get the department/municipality of the inscription (Acuerdo 55-2014), an ordinary code; the code for a consular birth registered abroad is delegated by Decreto 39-2010 to the RENAP Directorship and is not published. | Possible false negatives for remainder-10 CUIs and for a consular code outside `01-22`; Petatán follows INE. | Use `kind = "cui"`; verify in RENAP/SAT. |
| GTM | NIT | SAT's FEL annex documents length, `K` and hyphen, but the check digit for legacy NITs and the CUI-prefix relationship rely on the published annex + FAQ. | A 9-digit CUI prefix passes the NIT check because the weights are mirrors. | Verify in SAT. |
| HND | DNI, RTN | The SAR does not publish the RTN verifier (124 supplier RTNs and a weight/modulus sweep showed no check digit). The DNI internal layout is observed, not normative. | Shape only; the RTN's last digit is free. | Verify in SAR. |
| MEX | CURP | Position 17 is `0-9` up to 1999 and `A-J` from 2000; a hypothetical K-Z CURP issued under the 2006 text would be rejected. The offensive-name catalogue is not published, so it is not applied. Future birth dates are not rejected. | Narrow false negatives/positives. | Verify in RENAPO. |
| MEX | RFC | 12/13 characters only; the 10-character legacy form is rejected. Dates are read as 20xx (the SAT does not encode the century). The generic RFCs are accepted as an explicit exception. | Legacy 10-character RFCs are false negatives. | Verify in SAT. |
| NIC | Cédula | The municipality code is not checked: no official CSE list of the 153 codes exists and third-party lists contradict each other (Mulukukú, `888`, Bocay/El Cuá). The mod-23 letter is confirmed by independent validators, not by a CSE publication. | A nonexistent municipality can pass. | Verify in CSE. |
| PAN | Cédula/RUC | The `EE`, `SB` and `E-NT` series have no published format and are rejected. The DGI DV is assigned per taxpayer type and changes on registration, so it is not computed; the DGI's *Ficha Técnica* also sizes the RUC field at 5-21 alphanumeric characters, wider than the printed shapes accepted here. | Possible false negatives for `EE`/`SB` and for an alphanumeric RUC outside the documented shapes; RUC shape only. | Verify in Tribunal Electoral / DGI. |
| PER | RUC | The check digit and prefixes `10`/`15`/`20` are public, but `17` is only documented by implementations. | A `17` RUC is accepted; an undocumented prefix is rejected. | Verify in SUNAT. |
| PRY | CI, RUC | The CI has no published length or check digit; 6-8 digits is observed. An 8-digit CI is inferred as a RUC unless `kind = "ci"`. RUCs with a letter before the check digit are not supported. | The 8-digit CI is a false negative by inference. | Use `kind = "ci"`; verify in DNIT/Policía Nacional. |
| SLV | DUI, NIT | The DUI and NIT algorithms are reconstructed from implementations; the two NIT weight tables and the `<= 100` cut-off are reverse engineered. Below the cut-off both tables are accepted, which raises the false-positive rate. | A 1-in-10 wrong NIT below the cut-off can pass. | Verify in MH/DGII. |
| URY | CI, RUT | The DGI withholds both algorithms; the weights come from public implementations. The RUT `001` group is confirmed by OECD/TIN Validate but the scanned MEF response could not be read. A RUT whose check digit is 10 is unrepresentable. | A RUT with another middle group would be a false negative. | Verify in DGI. |
| VEN | RIF | The SENIAT does not publish the letter table; `C = 12` is inferred from the 2015 comuna migration and public tables. | A new letter would be rejected. | Verify in SENIAT. |

## Policy for callers

- Use `kind` for ambiguous documents: `COL` NIT (`"nit"`), `CRI` NITE vs cédula
  jurídica (`"nite"`/`"cedula_juridica"`), `PRY` CI/RUC (`"ci"`/`"ruc"`), `GTM`
  12-digit CUI (`"cui"`), `SLV` DUI/NIT (`"dui"`/`"nit"`), `URY` CI/RUT
  (`"id_card"`/`"rut"`).
- Treat `Ok` as a format result, not as proof of existence. For KYC, invoicing
  or payments, query the registry.
- When a user reports a valid number that the crate rejects, ask for a synthetic
  equivalent; never copy the real number into code, tests, docs or issues.

## Where to verify

Portals, not deep links: they change. The per-crate READMEs carry the exact
sources that were checked on the date shown there.

| Country | Registry |
|---|---|
| ARG | ARCA/AFIP, constancia de inscripción: https://www.afip.gob.ar/constanciadeinscripcion/ |
| BOL | SIN/SIAT, "Verifica NIT": https://siatinfo.impuestos.gob.bo/ |
| BRA | Receita Federal, CPF/CNPJ: https://www.gov.br/receitafederal/ |
| CHL | SII, consulta de RUT: https://www.sii.cl/ |
| COL | DIAN, consulta de RUT: https://www.dian.gov.co/ |
| CRI | TSE / Registro Nacional: https://www.tse.go.cr/ |
| CUB | MININT (no public online service) |
| DOM | DGII, consulta RNC/cédula: https://dgii.gov.do/ |
| ECU | SRI, consulta RUC: https://www.sri.gob.ec/ |
| GTM | SAT, consulta CUI/NIT: https://portal.sat.gob.gt/portal/consulta-cui-nit |
| HND | SAR, verificación de RTN: https://www.sar.gob.hn/registro-tributario-nacional-rtn |
| MEX | SAT RFC / RENAPO CURP: https://www.sat.gob.mx/ · https://www.gob.mx/curp/ |
| NIC | CSE, consulta de cédula: https://www.cse.gob.ni/ |
| PAN | DGI DV / Tribunal Electoral: https://dgi.mef.gob.pa/DV · https://www.tribunal-electoral.gob.pa/ |
| PER | SUNAT RUC / RENIEC DNI: https://www.sunat.gob.pe/ · https://identidad.reniec.gob.pe/ |
| PRY | DNIT / Policía Nacional: https://www.dnit.gov.py/ · https://www.policianacional.gov.py/ |
| SLV | MH NIT / RNPN DUI: https://www.mh.gob.sv/ · https://www.rnpn.gob.sv/ |
| URY | DGI RUT / DNIC: https://www.gub.uy/direccion-general-impositiva/ |
| VEN | SENIAT RIF: http://www.seniat.gob.ve/ |
