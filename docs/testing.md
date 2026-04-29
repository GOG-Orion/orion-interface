# Testing

## What to verify

- Version parsing and comparison
- Release metadata normalization
- Install-root discovery and override behavior
- Digest verification
- Payload validation
- Closed-client install blocking

## Local commands

```bash
pnpm typecheck
pnpm build
cargo test
```

## Environment note

Linux Tauri tests require the normal GTK and WebKit system libraries.
If those libraries are missing, the backend test run can fail before it reaches the project code.
