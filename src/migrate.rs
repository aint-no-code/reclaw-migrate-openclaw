use thiserror::Error;

use crate::{OpenClawConfig, ReclawConfig};

pub fn migrate_config(source: OpenClawConfig) -> Result<ReclawConfig, MigrationError> {
    validate_auth_secrets(&source)?;
    validate_port(source.port)?;

    Ok(ReclawConfig {
        host: source.host,
        port: source.port,
        gateway_token: source.gateway_token,
        gateway_password: source.gateway_password,
        max_payload_bytes: source.max_payload_bytes,
        max_buffered_bytes: source.max_buffered_bytes,
        handshake_timeout_ms: source.handshake_timeout_ms,
        tick_interval_ms: source.tick_interval_ms,
        cron_enabled: source.cron_enabled,
        cron_poll_ms: source.cron_poll_ms,
        cron_runs_limit: source.cron_runs_limit,
        db_path: source.db_path,
        auth_max_attempts: source.auth_max_attempts,
        auth_window_ms: source.auth_window_ms,
        runtime_version: source.runtime_version,
        log_filter: source.log_level,
        json_logs: source.json_logs,
    })
}

fn validate_auth_secrets(source: &OpenClawConfig) -> Result<(), MigrationError> {
    match (&source.gateway_token, &source.gateway_password) {
        (Some(token), Some(password))
            if !token.trim().is_empty() && !password.trim().is_empty() =>
        {
            Err(MigrationError::ConflictingAuthSecrets)
        }
        _ => Ok(()),
    }
}

fn validate_port(port: Option<u16>) -> Result<(), MigrationError> {
    if let Some(value) = port {
        if value == 0 {
            return Err(MigrationError::InvalidPort);
        }
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("openclaw config cannot contain both gatewayToken and gatewayPassword")]
    ConflictingAuthSecrets,

    #[error("openclaw config contains invalid port value (must be > 0)")]
    InvalidPort,
}
