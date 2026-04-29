# Install Flow

## Default rule

Orion installs integrations only when the GOG client is closed.

## Flow

1. The UI checks install readiness.
2. If the client is running, the UI blocks install and shows a close-the-client message.
3. If the client is closed, the UI asks the backend to download the current release.
4. The backend validates the archive and copies the payload into the install root.
5. The backend records the installed release tag after a successful install.

## Safety checks

- Reject symlinks.
- Reject executable and script payloads.
- Reject digest mismatches when the release includes a SHA-256 checksum.

## Failure handling

- The UI should surface the backend error message.
- Temporary download files should be removed even when install fails.
