use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct OpenClawConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub gateway_token: Option<String>,
    pub gateway_password: Option<String>,
    pub max_payload_bytes: Option<usize>,
    pub max_buffered_bytes: Option<usize>,
    pub handshake_timeout_ms: Option<u64>,
    pub tick_interval_ms: Option<u64>,
    pub cron_enabled: Option<bool>,
    pub cron_poll_ms: Option<u64>,
    pub cron_runs_limit: Option<usize>,
    pub db_path: Option<String>,
    pub auth_max_attempts: Option<u32>,
    pub auth_window_ms: Option<u64>,
    pub runtime_version: Option<String>,
    pub log_level: Option<String>,
    pub json_logs: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReclawConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_payload_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_buffered_bytes: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake_timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_interval_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_poll_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_runs_limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_max_attempts: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_window_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_logs: Option<bool>,
}
