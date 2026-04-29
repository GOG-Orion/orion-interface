# Security Policy

Orion treats integration releases as data-only payloads.

## Release rules

- Do not execute scripts from downloaded archives.
- Reject archives that include symlinks, `.bat`, `.cmd`, `.ps1`, `.exe`, or similar executable payloads.
- Prefer release assets that are ZIP archives containing only plugin files and manifests.

## Reporting issues

- Report unsafe release content through the normal repository issue workflow.
- If a release should be blocked, document the repository, tag, asset name, and observed payload type.

## Hardening notes

- Install roots can be overridden by the user.
- The installer should continue to validate `manifest.json` before copying files.
