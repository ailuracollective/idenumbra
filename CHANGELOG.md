# Changelog

## [0.1.0-alpha.2](https://github.com/ailuracollective/idenumbra/compare/v0.1.0-alpha.1...v0.1.0-alpha.2) (2026-11-20)


### Features

* **arg:** return Validation with components ([655b73d](https://github.com/ailuracollective/idenumbra/commit/655b73d79f62058530b0105c878d5e941ab1aa6d))
* **bol:** return Validation with components ([a6e852e](https://github.com/ailuracollective/idenumbra/commit/a6e852e57384b227cb91310463a6d7b1f313258b))
* **bra:** return Validation with components ([7e93c00](https://github.com/ailuracollective/idenumbra/commit/7e93c00158ef41bdb1d0ad0907fa37b2becad360))
* classify all compatible document kinds within a country ([#85](https://github.com/ailuracollective/idenumbra/issues/85)) ([f86d9e0](https://github.com/ailuracollective/idenumbra/commit/f86d9e0698ff03544fc8d22910740ff07dbe4c61))
* **cri:** return Validation with components ([94d4d2f](https://github.com/ailuracollective/idenumbra/commit/94d4d2f2ec5f6be1a9eef823a21529edcd5e3baf))
* **ecu:** expose components via Validation ([525f05b](https://github.com/ailuracollective/idenumbra/commit/525f05b57b088054975ecf7adb71ee191a01ff6c))
* **gitignore:** add .gitignore for CodeGraph data files ([affe3e7](https://github.com/ailuracollective/idenumbra/commit/affe3e74ddab78a488cfcfe0ff2fc7670b66fd92))
* **gtm:** return Validation with components ([ea29cd2](https://github.com/ailuracollective/idenumbra/commit/ea29cd2afae24c9891f09af950c178ab309eb906))
* **hnd:** return Validation with components ([a5b6492](https://github.com/ailuracollective/idenumbra/commit/a5b6492d83082eb63662a33fb3defd33c0347821))
* **mex:** return Validation with components ([da224f9](https://github.com/ailuracollective/idenumbra/commit/da224f98eda2ccac42a09f619f4af075856f91fa))
* **nic:** return Validation with components ([c01e8d5](https://github.com/ailuracollective/idenumbra/commit/c01e8d56303ad4bf72000b2537109de0ad04ecda))
* **pan:** return Validation with components ([0603273](https://github.com/ailuracollective/idenumbra/commit/060327303f64df664fc70cc58b35e90601487e75))
* **per:** return Validation with components ([7edd0d6](https://github.com/ailuracollective/idenumbra/commit/7edd0d694d00c0db920bdbed91ea18bd046e04a6))
* rich Validation result type and components extraction framework ([#4](https://github.com/ailuracollective/idenumbra/issues/4)) ([4fae0aa](https://github.com/ailuracollective/idenumbra/commit/4fae0aac7e40fcfdd8a8333fa92c472c082519d6))
* **std:** add allocation-free batch validation ([#83](https://github.com/ailuracollective/idenumbra/issues/83)) ([6bd60a2](https://github.com/ailuracollective/idenumbra/commit/6bd60a25060aac7013f98a0c2190a2bcfdd75433))
* **std:** expose country components via Idenumbra ([f267080](https://github.com/ailuracollective/idenumbra/commit/f2670808309c0d778ffd0ce2fa0302e4d5f0af5d))
* **std:** identify candidate countries from validation rules ([#81](https://github.com/ailuracollective/idenumbra/issues/81)) ([12b9cc7](https://github.com/ailuracollective/idenumbra/commit/12b9cc76980ee77987a73cfd194cbd8c6e2a3a3d))
* update documentation with contents and API references ([f3e8b31](https://github.com/ailuracollective/idenumbra/commit/f3e8b31e2d7240f7dea2aa310a3bf4d10cc97189))
* **ury:** return Validation with components ([560a7d9](https://github.com/ailuracollective/idenumbra/commit/560a7d954488ce8b8951fa7ceac81dbb990beb8d))
* **ven:** return Validation with components ([dc3e150](https://github.com/ailuracollective/idenumbra/commit/dc3e15091784f27efe68f1edb6ae1371c487debd))


### Bug Fixes

* **col:** reject all-zero placeholders in the typed kinds ([eff1f90](https://github.com/ailuracollective/idenumbra/commit/eff1f906c8e9411a9cb9c333222a749e929aec8a))
* **cri:** reject an all-zero NITE in the typed kind ([87ed6b0](https://github.com/ailuracollective/idenumbra/commit/87ed6b040aac8aaccdb58f613cba218bd781de6c))
* **dom:** accept formatted cedulas in the typed kind ([d268d32](https://github.com/ailuracollective/idenumbra/commit/d268d32a9942acb67934be76beb58af453ad541e))
* **per:** reject an all-zero DNI in the typed kind ([72519ef](https://github.com/ailuracollective/idenumbra/commit/72519ef75844bba68cc8816087e2824f83127320))
* **pry:** accept separators for the CI kind ([7f30f53](https://github.com/ailuracollective/idenumbra/commit/7f30f5345d97cb8f8d9f327da0fa2c29ba63caf0))
* **std:** report the validated kind in Idenumbra::validate ([8283653](https://github.com/ailuracollective/idenumbra/commit/82836537ce61d65067e09b6730a97c43887b6788))

## 0.1.0-alpha.1 (2026-11-18)


### Features

* **bol:** validate NIT control codes 01/02/04 ([0aa7a94](https://github.com/ailuracollective/idenumbra/commit/0aa7a944e01dcae48f43437504c3468e6a1d75a8))
* **cri:** add an explicit 10-digit nite kind ([2487a8e](https://github.com/ailuracollective/idenumbra/commit/2487a8e8557deccdc390b3ba0a7a42726602dd48))
* **cri:** support cédula jurídica and reject leading zeros ([b55b706](https://github.com/ailuracollective/idenumbra/commit/b55b70652e936a959a98dff9422b838fa67d25c3))
* initial commit of the idenumbra workspace ([c0335db](https://github.com/ailuracollective/idenumbra/commit/c0335dbc6dcfa88bcd4d476742cf78296ff5d18d))
* **pan:** accept the DGI RUT NT series ([6db7c1e](https://github.com/ailuracollective/idenumbra/commit/6db7c1e51fda7c9be93e4547453b9f72adfb87ac))

### Bug Fixes:

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
