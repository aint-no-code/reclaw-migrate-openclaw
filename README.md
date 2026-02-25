# Reclaw Migrate OpenClaw

`reclaw-migrate-openclaw` converts OpenClaw runtime config JSON into Reclaw Core static config TOML.

## Usage

```bash
cargo run -- --input ./openclaw.config.json --output /etc/reclaw/config.toml
```

Dry run:

```bash
cargo run -- --input ./openclaw.config.json --dry-run
```

## Validation Rules

- Exactly zero or one auth secret (`gatewayToken` or `gatewayPassword`)
- `port` must be greater than `0` when present

## Quality Gates

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
