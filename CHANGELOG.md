# Changelog

## 0.1.0-alpha.1 (2026-11-18)


### Features

* **bol:** validate NIT control codes 01/02/04 ([0aa7a94](https://github.com/ailuracollective/idenumbra/commit/0aa7a944e01dcae48f43437504c3468e6a1d75a8))
* **cri:** add an explicit 10-digit nite kind ([2487a8e](https://github.com/ailuracollective/idenumbra/commit/2487a8e8557deccdc390b3ba0a7a42726602dd48))
* **cri:** support cédula jurídica and reject leading zeros ([b55b706](https://github.com/ailuracollective/idenumbra/commit/b55b70652e936a959a98dff9422b838fa67d25c3))
* initial commit of the idenumbra workspace ([c0335db](https://github.com/ailuracollective/idenumbra/commit/c0335dbc6dcfa88bcd4d476742cf78296ff5d18d))
* **pan:** accept the DGI RUT NT series ([6db7c1e](https://github.com/ailuracollective/idenumbra/commit/6db7c1e51fda7c9be93e4547453b9f72adfb87ac))


### Bug Fixes

* **bol:** constrain NITs to 9-11 digits and add control code 03 ([5f8625c](https://github.com/ailuracollective/idenumbra/commit/5f8625c9c3b352ded31149dbdd39c2a70c3485d7))
* **bra:** document and test alphanumeric CNPJ rules ([7d710ce](https://github.com/ailuracollective/idenumbra/commit/7d710cec7f78101076a29ce0fa3b7f9fe2b7596b))
* **col:** accept 10-11 digit NIT bodies ([b6bc55a](https://github.com/ailuracollective/idenumbra/commit/b6bc55acfb0e7bf4d55c441f78a8caec50aaeed2))
* **ecu:** require fixed establishments and relax company check digits ([f4ab615](https://github.com/ailuracollective/idenumbra/commit/f4ab615a43d10d65370e3f5acf696771cab5c96f))
* **gtm:** correct NIT/CUI validity dates and department region errors ([f4016b1](https://github.com/ailuracollective/idenumbra/commit/f4016b1863d2b7f887f04fc80998768fe06f3e5b))
* **gtm:** depurate printed CUI and NIT forms before validating ([3bd589f](https://github.com/ailuracollective/idenumbra/commit/3bd589f073d4a55ab559f648d2db8a4d3224aef1))
* **gtm:** try the 12-digit CUI layout before the NIT rule ([ad9ae04](https://github.com/ailuracollective/idenumbra/commit/ad9ae04e6b2d34091c10843ffc1b9a26fe2774ba))
* **gtm:** use the SAT FEL check-digit rule and accept 12-digit CUIs ([cc5a5db](https://github.com/ailuracollective/idenumbra/commit/cc5a5db6ff76ca31dfaa044d5285adcedc24d921))
* **mex:** enforce CURP consonant positions and allow non-binary X ([83a32b0](https://github.com/ailuracollective/idenumbra/commit/83a32b0795c03c779171dd0293517c56004c20c3))
* **mex:** restrict CURP position 17 to A-J and use synthetic fixtures ([277e098](https://github.com/ailuracollective/idenumbra/commit/277e098638294c46b0ffeab131df94340147d29d))
* **nic:** infer passports and keep the id_card kind strict ([6adbf1b](https://github.com/ailuracollective/idenumbra/commit/6adbf1b9a13c42a75696ad2ee860ccd3516a6820))
* **pan:** reject a passport or NT under the wrong document kind ([4b4b13c](https://github.com/ailuracollective/idenumbra/commit/4b4b13c7fed9d27acefe4aaf2d5476b0b170deed))
* **pry:** require a 3-8 digit RUC body and reject zero CIs ([7318e75](https://github.com/ailuracollective/idenumbra/commit/7318e757461673ec37b3681e53ac019ca6d78e4a))
* **slv:** accept both NIT weight tables below the legacy cut-off ([857666b](https://github.com/ailuracollective/idenumbra/commit/857666bef52499a49aeda66483e41a78ca1928a6))
* **ven:** reject all-zero CI for the explicit kind ([f8a6730](https://github.com/ailuracollective/idenumbra/commit/f8a6730aba6453301f75d57e1d2f35ac6a290361))
