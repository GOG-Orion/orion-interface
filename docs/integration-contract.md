# Integration Contract

Orion-managed integrations should follow a predictable release shape.

## Required metadata

- Repository name
- Latest release tag
- Download URL for the archive asset
- SHA-256 digest when the release provides one

## Expected release contents

- A manifest file named `manifest.json`
- A payload tree that can be copied into the integration directory
- No scripts, executables, or symlinks

## Naming rules

- Asset names should be stable across releases.
- The backend should fail loudly when it cannot map a repository to a supported integration.
- The frontend should not invent release metadata that the backend does not provide.

## Compatibility

- Upstream repositories may not match this contract yet.
- The sync workflow can normalize metadata, but the runtime installer should still reject unsafe payloads.
