# Configuration

## Install root

Orion Interface uses an install root for extracted integration files.

- The app first looks for known GOG Galaxy plugin locations.
- The user can override the install root from the configuration UI.
- An override should win over discovery when it is set.

## Environment override

`ORION_INSTALL_ROOT` can be used to force the install root for local testing or non-standard setups.

## UI behavior

- Configuration changes should be reflected in the app state immediately.
- The UI should show whether the install root came from discovery or from a user override.

## Data file

- Shared app configuration is stored by the backend.
- Installation-specific state should stay separate from version metadata.
