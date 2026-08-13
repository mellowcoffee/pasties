use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::serde_default;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub server:       Server,
    pub database:     Database,
    pub limits:       Limits,
    pub registration: Registration,
    pub session:      Session,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    pub bind: SocketAddr,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Database {
    pub url:             String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Moderation {
    pub max_pages_per_user:            u32,
    pub login_attempts_per_minute:     u32,
    pub registrations_per_hour_per_ip: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Limits {
    pub bio_max_len:        usize,
    pub password_min_len:   usize,
    pub password_max_len:   usize,
    pub avatar_url_max_len: usize,
    pub html_max_bytes:     usize,
    pub css_max_bytes:      usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Registration {
    pub require_invite: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Session {
    pub cookie_name:   String,
    pub lifetime_days: u32,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading config {}", path.display()))?;
        let config: Self = toml::from_str(&text)
            .with_context(|| format!("parsing config {}", path.display()))?;
        Ok(config)
    }
}

serde_default! {
    max_connections: u32 = 5;
}

impl Default for Moderation {
    fn default() -> Self {
        Self {
            max_pages_per_user:            5000,
            login_attempts_per_minute:     5,
            registrations_per_hour_per_ip: 5,
        }
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            bio_max_len:        512,
            password_min_len:   6,
            password_max_len:   128,
            avatar_url_max_len: 512,
            html_max_bytes:     262144,
            css_max_bytes:      131072,
        }
    }
}

impl Default for Registration {
    fn default() -> Self {
        Self {
            require_invite: true,
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            cookie_name:   "session".to_owned(),
            lifetime_days: 365,
        }
    }
}
