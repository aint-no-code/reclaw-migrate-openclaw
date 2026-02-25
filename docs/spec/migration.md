# Migration Spec

## Input

OpenClaw JSON config (`camelCase` keys).

## Output

Reclaw static config TOML with the same operational fields and these mappings:

- `logLevel` -> `logFilter`

## Invariants

- Reject when both `gatewayToken` and `gatewayPassword` are present and non-empty.
- Reject when `port == 0`.
- `--output` is required unless `--dry-run` is enabled.
