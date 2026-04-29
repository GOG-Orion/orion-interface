# Sync Policy

## Goal

Keep Orion-managed integration metadata aligned with upstream releases without trusting upstream payloads blindly.

## Rules

- Refresh release metadata in a controlled workflow.
- Open a pull request when metadata changes.
- Review repository mapping changes before merging.
- Keep the runtime installer responsible for safety checks.

## Workflow responsibilities

- The sync workflow updates release tags, URLs, and digests.
- The backend still validates payload integrity at install time.
- The frontend should only consume metadata that the backend can honor.
