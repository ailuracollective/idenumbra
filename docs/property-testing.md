# Property-based testing

Idenumbra uses [proptest](https://docs.rs/proptest/) for Rust property-based tests. The dependency is declared in the workspace and enabled as a dev dependency in `idenumbra-core`. Production code remains dependency-free and `no_std` compatible.

## Run tests

```sh
cargo test --workspace
cargo test -p idenumbra-core --test property_smoke
```

## Writing country-specific properties

Add `proptest.workspace = true` under `[dev-dependencies]` in the relevant country crate. Keep strategies close to their country's document rules. Generate valid identifiers using independent specifications or reference vectors; do not use the production validator as the sole oracle. Add invalid inputs, boundary cases, determinism, and panic-resistance properties where applicable. Do not assume every mutation invalidates a document.

## Reproducing failures

Proptest shrinks failing inputs and reports minimal counterexamples. By default its failure persistence mechanism stores regression seeds in a `proptest-regressions` file associated with the test source (when supported by the test environment). Preserve those regression files when failures are discovered. Re-run the same test with `cargo test -p <crate> <test_name> -- --nocapture` to inspect the failure; use `PROPTEST_CASES` to increase the case count during exploratory runs.

Country adoption is tracked separately (starting with #15 for `arg`).
