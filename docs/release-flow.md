# Release Flow

## Source of truth

- GitHub releases provide the downloadable archive and release metadata.
- The backend compares installed and latest versions using parsed version values, not string sorting.

## Expected release handling

- Prefer a zip asset for installation.
- Preserve the release digest when GitHub provides a SHA-256 checksum.
- Record the installed tag only after the install succeeds.

## Version display

- The frontend version should come from the app package metadata.
- Release and install state should be shown separately from app version state.
