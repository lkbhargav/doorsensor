use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Gmail {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    pub gmail: Gmail,
    // in milliseconds
    pub ping_interval: u32,
    pub email_alert: bool,
    pub db_path: String,
    pub enable_feedback: bool,
    pub slack_token: String,
}

macro_rules! parse_env {
    ($key:expr, $mess:expr) => {
        env::var($key).expect($mess)
    };
}

impl EnvironmentVariables {
    pub fn init() -> Result<Self> {
        let username = parse_env!("DS_USERNAME", "DS_USERNAME is required but not found");
        let password = parse_env!(
            "DS_APP_PASSWORD",
            "DS_APP_PASSWORD is required but not found"
        );
        let ping_interval = parse_env!(
            "DS_PING_INTERVAL",
            "DS_PING_INTERVAL is required but not found"
        );
        let ping_interval = ping_interval.parse::<u32>()?;
        let email_alert = parse_env!("DS_EMAIL_ALERT", "DS_EMAIL_ALERT is required but not found");
        let email_alert = email_alert.parse::<bool>()?;
        let db_path = parse_env!("DS_DB_PATH", "DS_DB_PATH is required but not found");
        let enable_feedback = parse_env!(
            "DS_ENABLE_FEEDBACK",
            "DS_ENABLE_FEEDBACK is required but not found"
        );
        let enable_feedback = enable_feedback.parse::<bool>()?;

        let slack_token = parse_env!("DS_SLACK_TOKEN", "DS_SLACK_TOKEN is required but not found");

        Ok(EnvironmentVariables {
            gmail: Gmail { username, password },
            ping_interval,
            email_alert,
            db_path,
            enable_feedback,
            slack_token,
        })
    }
}
