# Feature: CLI codegen migration

Migrate from publishing one crate per country to a shadcn-like CLI that copies validator source into consumer projects.

## Specs

- S1: "migraremos a una estretegia de cli, este cli es instalara por proyecto (no global no sistema) y correra bajo cargo para poder instalar los validadores"
- S2: "se planea algo como idenumbra add ECU ARG"
- S3: CLI is a dedicated crate published as `idenumbra` (binary `idenumbra`), not in core or std.
- S4: `idenumbra-core` remains published (contract); country crates (`idenumbra-<code>`) stay as internal resources with `publish = false`, not removed.
- S5: Config file is `idenumbra.toml` (TOML), idempotent, tracks installed countries and version.
- S6: Generated code is `no_std`-compatible by default; supports std consumers.
- S7: Generated code includes a mini-dispatcher (`infer`, `validate`, `validate_with`, `supported_countries`) plus per-country functions.
- S8: No tests are copied into the consumer project.
- S9: Conflicts overwrite existing files (`over`).
- S10: `--dry-run`/diff preview supported.
- S11: Commands: `add`, `list`, `remove`, `update`, `diff`.
- S12: Invocation: `cargo run -p idenumbra --`, alias `cargo idenumbra`, ephemeral `cargo run --git`, `cargox idenumbra`, `cargo-run-bin`.
- S13: Country crates are internal resources (not published); listed separately in README.

## Tasks

- T1: Scaffold `crates/cli` crate with clap CLI and command stubs (add/list/remove/update/diff) — S3,S11 — done
- T2: Implement `add <CODE...>` copying country validator sources into consumer project — S1,S2,S6,S7,S9 — done
- T3: Generate `idenumbra.toml` manifest on add — S5,S10 — done
- T4: Implement `list`, `remove`, `diff --dry-run` — S9,S10,S11 — done
- T5: Implement `update`/`sync` from GitHub release tarball (fallback to local) — S4,S5 — done
- T6: (attempted) Inline country validators into std; reverted — country crates stay internal — dropped
- T7: Update README, AGENTS, docs for new model — S4,S13 — done
- T8: Modularize CLI into `cli`/`commands`/`util` modules — done
- T9: Add E2E tests (assert_cmd + tempfile) for add/list/remove/dry-run/unknown/multi — done
- T10: Add UX crates (console, indicatif, human-panic) — done
- T11: Mark country crates `publish = false`; yank published alphas — done
- T12: Rename orchestrator to `idenumbra-std`, CLI to `idenumbra` (alpha, breaking) — done
- T13: Add `.cargo/config.toml` alias and document cargo-run-bin/cargox/crgx — done

## Log

- L1: Original request verbatim: "necesitamos un cambio de enfoque, actualemente se publica un creates por cada pais, eso no es escalable a largo plazo, asi que migraremos a una estretegia de cli, este cli es instalara por proyecto (no global no sistema) y correra bajo cargo para poder instalar los validadores, se planea algo como idenumbra add ECU ARG, primero pulamos la idea antes de codificar"
- L2: Decision: codegen model (shadcn-style), CLI as dedicated crate, core stays published, country crates become internal resources (publish=false), not removed.
- L3: CLI renamed to `idenumbra` (crate), orchestrator renamed to `idenumbra-std`; alias `cargo idenumbra` added.
- L4: Published alpha versions of country crates yankear via cargo yank; complete deletion only via crates.io web UI (<72h).
