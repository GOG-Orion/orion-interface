# Orion Sync Policy

This document defines how Orion mirrors and curates upstream integration repositories.

## Model

- Orion may track upstream repositories directly.
- Orion may maintain Orion-owned mirrors when a stable release contract is required.
- Sync branches should be short-lived and purpose-specific.

## Branch flow

- Use `sync/<repo>/<date>` for upstream refresh work.
- Keep `feature/*` and `fix/*` branches separate from sync work.
- Promote sync branches only after build and validation pass.

## Review rules

- Preserve upstream history unless a deliberate rewrite is required.
- Do not remove upstream fixes without a documented reason.
- Update release metadata when an Orion mirror becomes the distribution source.

## Release rules

- A sync is not a release by default.
- Release promotion should happen only after verification, packaging, and documentation updates.
