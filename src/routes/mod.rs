use axum::routing::get;
use axum::Router;

use crate::routes::pages::handle_page;
use crate::routes::views::{login, register};

pub mod api;
pub mod middleware;
pub mod pages;
pub mod views;

pub fn router(state: crate::State) -> Router {
    Router::new()
        .route("/register", get(register(&state)))
        .route("/login", get(login()))
        .route("/{slug}", get(handle_page))
        .with_state(state)
}
