mod migrate;
mod model;

pub use migrate::{migrate_config, MigrationError};
pub use model::{OpenClawConfig, ReclawConfig};

#[cfg(test)]
mod tests {
    use crate::{migrate_config, MigrationError, OpenClawConfig};

    #[test]
    fn migration_maps_log_level_and_auth_token() {
        let source = OpenClawConfig {
            host: Some("0.0.0.0".to_owned()),
            port: Some(18_789),
            gateway_token: Some("token-123".to_owned()),
            gateway_password: None,
            max_payload_bytes: Some(10),
            max_buffered_bytes: Some(100),
            handshake_timeout_ms: Some(5000),
            tick_interval_ms: Some(1000),
            cron_enabled: Some(false),
            cron_poll_ms: Some(500),
            cron_runs_limit: Some(4),
            db_path: Some("/tmp/reclaw.db".to_owned()),
            auth_max_attempts: Some(3),
            auth_window_ms: Some(1000),
            runtime_version: Some("1.2.3".to_owned()),
            log_level: Some("debug".to_owned()),
            json_logs: Some(true),
        };

        let target = migrate_config(source).expect("migration should succeed");

        assert_eq!(target.gateway_token.as_deref(), Some("token-123"));
        assert_eq!(target.log_filter.as_deref(), Some("debug"));
        assert_eq!(target.host.as_deref(), Some("0.0.0.0"));
    }

    #[test]
    fn migration_rejects_dual_auth_secrets() {
        let source = OpenClawConfig {
            host: None,
            port: None,
            gateway_token: Some("a".to_owned()),
            gateway_password: Some("b".to_owned()),
            max_payload_bytes: None,
            max_buffered_bytes: None,
            handshake_timeout_ms: None,
            tick_interval_ms: None,
            cron_enabled: None,
            cron_poll_ms: None,
            cron_runs_limit: None,
            db_path: None,
            auth_max_attempts: None,
            auth_window_ms: None,
            runtime_version: None,
            log_level: None,
            json_logs: None,
        };

        let result = migrate_config(source);
        assert!(matches!(
            result,
            Err(MigrationError::ConflictingAuthSecrets)
        ));
    }

    #[test]
    fn migration_rejects_invalid_port() {
        let source = OpenClawConfig {
            host: None,
            port: Some(0),
            gateway_token: None,
            gateway_password: None,
            max_payload_bytes: None,
            max_buffered_bytes: None,
            handshake_timeout_ms: None,
            tick_interval_ms: None,
            cron_enabled: None,
            cron_poll_ms: None,
            cron_runs_limit: None,
            db_path: None,
            auth_max_attempts: None,
            auth_window_ms: None,
            runtime_version: None,
            log_level: None,
            json_logs: None,
        };

        let result = migrate_config(source);
        assert!(matches!(result, Err(MigrationError::InvalidPort)));
    }
}
