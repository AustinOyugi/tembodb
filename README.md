# TemboDB

Experimental, PostgreSQL‑inspired database engine written in Rust. TemboDB currently does not support SQL; instead, it exposes low‑level data structures and functionality directly, avoiding the overhead of SQL parsing and translation.

Status: early prototype/WIP.

## Features

- Memory contexts: hierarchical memory management for controlled allocation and context switching (`src/memory`).
- Storage manager bootstrap: initializes base directories and checks readiness (`src/storage`).
- Environment bootstrap: orchestrates first‑run initialization (storage + system catalog scaffolding) (`src/environment`, `src/catalog`).
- Config loading: reads simple `key=value` config from `tembodata/tembodb.conf` (`src/config`).
- Logging: uses `env_logger` with standard `RUST_LOG` levels.

## Quick Start

Prerequisites:

- Rust toolchain (`rustup`), stable channel (see `rust-toolchain.toml`).
- A `tembodata/tembodb.conf` file with a valid base path.

Clone and build:

```bash
git clone <your-repo-url> tembodb
cd tembodb
cargo build
```

Create config (example):

```bash
mkdir -p tembodata
cat > tembodata/tembodb.conf <<'EOF'
file_path=/absolute/path/to/tembodata
EOF
```

Run with logs:

```bash
RUST_LOG=info cargo run
```

On first run, TemboDB enters bootstrap mode and creates required storage directories under `file_path` (e.g., `/storage`, `/base`, `/global`) and initializes system descriptors.

## Configuration

TemboDB reads `tembodata/tembodb.conf` in simple `key=value` format.

Required keys:

- `file_path`: absolute base directory for storage initialization and data files.

Example:

```conf
file_path=/Users/you/path/to/tembodata
```

## Project Structure

- `Cargo.toml`: crate metadata and dependencies.
- `src/`
  - `main.rs`: entry point; initializes logging, configs, memory, and bootstrap.
  - `config/`: config loader and accessors.
  - `environment/`: environment readiness checks and bootstrap flow.
  - `memory/`: memory context, registry, and setup utilities.
  - `storage/`: storage layout and storage manager (directory bootstrap, checks).
  - `catalog/`: system catalog scaffolding for bootstrap.
  - `constants/`: global constants and flags.
- `tembodata/`: local configuration and (eventual) data root.

## Development

- Build: `cargo build`
- Test: `cargo test`
- Run with logs: `RUST_LOG=info cargo run`

Conventional `rustfmt`/`clippy` can be added; if you want, run:

```bash
rustup component add clippy rustfmt
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Source file header (SPDX style):

```text
// Copyright 2026 Austin Oyugi
// SPDX-License-Identifier: Apache-2.0
```

## Contributing

Issues, ideas, and PRs are welcome. Please include:

- Clear problem statement or motivation.
- Minimal reproduction (if a bug).
- Tests where feasible and concise documentation for new behavior.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE` for terms and `NOTICE` for attribution.

---

TemboDB aims to explore database internals with a pragmatic, Rust‑first approach.
