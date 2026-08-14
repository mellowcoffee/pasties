#![allow(dead_code)]

use std::sync::Arc;

use anyhow::Context;
use axum::routing::get;
use axum::Router;
use snof::SnowflakeGenerator;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::config::Config;
use crate::database::init_database;
use crate::error::AppError;
use crate::model::pages::{self, CreatePage};
use crate::model::users::{self, CreateUser};

mod cli;
mod config;
mod database;
mod error;
mod model;
mod routes;
mod utility;
mod validation;

#[derive(Debug, Clone)]
pub struct State {
    pub pool:      PgPool,
    pub snowflake: Arc<SnowflakeGenerator>,
    pub config:    Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_owned());
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

    // test(state.clone()).await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .merge(routes::pages::router(state));
    // .route("/register", get(register_get).post(register))
    // .route("/login", get(login_get).post(login))
    // .route("/logout", post(logout))
    // .with_state(state);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn test(state: State) -> Result<(), AppError> {
    let user = users::create_user(
        CreateUser {
            username:         "admin".to_owned(),
            password:         "password".to_owned(),
            password_confirm: "password".to_owned(),
            invite_code:      String::new(),
        },
        &state,
    )
    .await?;

    let _page = pages::create_page(
        CreatePage {
            slug:     "first".to_owned(),
            owner_id: user.id,
            html:     "<b>Hello from First!</b>".to_owned(),
            css:      String::new(),
        },
        &state,
    )
    .await?;
    Ok(())
}
