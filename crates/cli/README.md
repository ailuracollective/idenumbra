# idenumbra-cli

CLI for installing idenumbra validators into a project via codegen (shadcn-style).

## Usage

```bash
# From inside the idenumbra repository:
cargo run -p idenumbra-cli -- add ECU ARG

# Ephemeral, from any project:
cargo run --git https://github.com/ailuracollective/idenumbra -- add ECU ARG
```

## Commands

- `add <CODE...>` — copy validator sources into the project.
- `list` — list installed validators.
- `remove <CODE...>` — remove installed validators.
- `update` — re-install all installed validators (fetches latest release tarball, falls back to local source).
- `diff <CODE...>` — preview changes (`--dry-run`).

## Configuration

`idenumbra.toml` at the project root tracks installed countries and version.

## Installation options (no global install)

### 1. Cargo alias (this repo)
```bash
cargo idenumbra add ECU ARG
```

### 2. cargo-run-bin (per-project, versioned)
```toml
# Cargo.toml of the consumer project
[package.metadata.bin]
idenumbra = { version = "0.1.0-alpha.4" }
```
```bash
cargo install cargo-run-bin
cargo bin idenumbra add ECU ARG
```

### 3. cargox / crgx (npx-style, no install)
```bash
cargox idenumbra-cli add ECU ARG
# or
crgx idenumbra@0.1.0-alpha.4 add ECU ARG
```
