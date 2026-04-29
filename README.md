# Orion Interface

Orion Interface is a Tauri desktop app for browsing, verifying, and installing GOG Galaxy 2.0 community integrations.

## What it does

- Lists available integrations
- Checks installed releases against the latest GitHub release
- Installs only when the GOG client is closed
- Validates archive integrity before copying files into the install root
- Supports translated UI strings and configuration overrides

## Requirements

- Node.js and `pnpm`
- Rust toolchain
- The normal Tauri platform dependencies for your OS

## Development

```bash
pnpm install
pnpm typecheck
pnpm build
cargo test
pnpm tauri dev
```

## Install behavior

- The app auto-discovers likely GOG Galaxy plugin paths when possible.
- A manual install-root override can be set from the UI or environment.
- Installation is blocked while the GOG client is running because the client can overwrite freshly installed files.

## Safety rules

- Archive payloads with scripts, executables, or symlinks are rejected.
- SHA-256 digests are verified when release metadata provides them.

## Project docs

- [Docs index](docs/README.md)
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
