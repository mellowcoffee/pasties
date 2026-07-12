#![allow(dead_code)]
#![allow(clippy::unreadable_literal)]

use std::sync::Arc;

use anyhow::Context;
use axum::{Router, routing::get};
use snof::SnowflakeGenerator;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::{config::Config, database::init_database};

mod cli;
mod config;
mod database;
mod error;
mod model;
mod routes;
mod utility;

#[derive(Clone)]
pub struct State {
    pub pool:      PgPool,
    pub snowflake: Arc<SnowflakeGenerator>,
    pub config:    Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = Config::load(&path.into())?;

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .context("connecting to database")?;
    init_database(&pool)
        .await
        .context("initializing database")?;

    let bind = config.server.bind;
    let state = State {
        pool,
        snowflake: Arc::new(SnowflakeGenerator::new()),
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        // .route("/register", get(register_get).post(register))
        // .route("/login", get(login_get).post(login))
        // .route("/logout", post(logout))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
