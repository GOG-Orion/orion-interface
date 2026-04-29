# Security

## Release payload rules

- Refuse scripts, executables, symlinks, and other unsafe payload types.
- Validate a SHA-256 digest when the release provides one.
- Fail closed when verification cannot be completed.

## Operational rules

- Prefer install with the GOG client closed to avoid overwrite races.
- Keep release metadata changes visible in sync pull requests.
- Review new asset types before adding them to the allowlist.

## Reporting

- Report install issues with the repository name, release tag, and backend error message.
- Report integrity issues with the digest and downloaded asset details.
