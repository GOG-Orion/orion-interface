# Orion Interface

Orion Interface is a lightweight Tauri desktop app for browsing, verifying, and installing GOG Galaxy 2.0 community integrations.

## Features

- Browse available integrations
- Check the installed version against the latest GitHub release
- Install releases into a configurable local install root
- Switch between translated UI strings
- View source, contributors, and configuration tabs

## Development

```bash
pnpm install
pnpm run typecheck
pnpm run test
pnpm run build
pnpm run tauri dev
```

## Notes

- The frontend version is derived from `package.json`.
- The install root can be configured from the UI.
- Linux Tauri builds still require the usual GTK/WebKit system libraries.
