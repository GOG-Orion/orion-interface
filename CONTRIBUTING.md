# Contributing

Orion Interface is a small community project, and contributions are welcome.

## Before you start

- Read the README and the docs index in `docs/README.md`.
- Check whether your change already exists in the current branch.
- Keep changes focused. Separate behavior changes from formatting or doc-only updates.

## Workflow

1. Fork or branch from the current repository.
2. Make the smallest change that solves the problem.
3. Run the relevant verification commands:
   - `pnpm typecheck`
   - `pnpm build`
   - `cargo test` when the local system libraries are available
4. Open a pull request with a clear summary of the change and the validation you ran.

## Code expectations

- Keep Tauri commands marked with `#[tauri::command]`.
- Prefer explicit error messages over silent failure.
- Avoid adding new dependencies unless they simplify the code materially.
- Do not change behavior without updating the matching docs or tests.

## Translation notes

- UI strings live in `src/components/utils/languages.json`.
- Keep new keys consistent across languages.
- If a translation is missing, add a fallback rather than leaving the UI blank.

## Testing notes

- Add backend tests for release parsing, validation, and install-flow rules.
- Add frontend checks when a change alters button behavior, routing, or visible state.
- Prefer targeted tests over broad mocks when the logic can be isolated.

## Reporting issues

- Include the exact steps to reproduce.
- Include the platform, runtime, and any relevant logs.
- If the bug affects installation, note whether GOG Galaxy was running.
