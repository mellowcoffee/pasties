use std::{net::SocketAddr, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

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
#[serde(deny_unknown_fields)]
pub struct Moderation {
    #[serde(default = "default_max_pages_per_user")]
    pub max_pages_per_user:            u32,
    #[serde(default = "default_login_attempts_per_minute")]
    pub login_attempts_per_minute:     u32,
    #[serde(default = "default_registrations_per_hour_per_ip")]
    pub registrations_per_hour_per_ip: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(default = "default_bio_max_len")]
    pub bio_max_len:        usize,
    #[serde(default = "default_password_min_len")]
    pub password_min_len:   usize,
    #[serde(default = "default_password_max_len")]
    pub password_max_len:   usize,
    #[serde(default = "default_avatar_url_max_len")]
    pub avatar_url_max_len: usize,
    #[serde(default = "default_html_max_bytes")]
    pub html_max_bytes:     usize,
    #[serde(default = "default_css_max_bytes")]
    pub css_max_bytes:      usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registration {
    #[serde(default = "default_require_invite")]
    pub require_invite: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    #[serde(default = "default_cookie_name")]
    pub cookie_name:   String,
    #[serde(default = "default_lifetime_days")]
    pub lifetime_days: u32,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading config {}", path.display()))?;
        let config: Config =
            toml::from_str(&text).with_context(|| format!("parsing config {}", path.display()))?;
        Ok(config)
    }
}

macro_rules! serde_default {
    ($($name:ident: $ty:ty = $val:expr;)*) => {
        paste::paste! {
            $(
                fn [<default_ $name>]() -> $ty { $val }
            )*
        }
    };
}

serde_default! {
    max_connections: u32 = 5;

    max_pages_per_user: u32 = 5000;
    login_attempts_per_minute: u32 = 5;
    registrations_per_hour_per_ip: u32 = 5;

    bio_max_len: usize = 512;
    password_min_len: usize = 6;
    password_max_len: usize = 128;
    avatar_url_max_len: usize = 512;
    html_max_bytes: usize = 262144;
    css_max_bytes: usize = 131072;

    require_invite: bool = true;

    cookie_name: String = "session".to_string();
    lifetime_days: u32 = 365;
}
