# Architecture

Orion Interface is split into a Tauri frontend and a Rust backend.

## Frontend

- Renders the integrations list and the supporting tabs.
- Verifies release state before install.
- Blocks installation when the GOG client is running.
- Sends explicit commands to the backend for release and configuration operations.

## Backend

- Resolves release metadata from GitHub.
- Compares installed and latest versions.
- Downloads, validates, extracts, and installs integration payloads.
- Stores the configurable install root.

## Data flow

1. The UI asks the backend for release state.
2. The backend resolves the release and compares versions.
3. The UI allows install only when a newer release exists and the GOG client is closed.
4. The backend downloads the archive, validates it, and copies the payload into the install root.

## Safety boundaries

- The backend refuses unsupported archive contents such as scripts, executables, and symlinks.
- The backend rejects checksum mismatches when a SHA-256 digest is present.
- The UI and backend both enforce the closed-client rule before installation.
