use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::model::pages::{self};
use crate::routes::pages::view::{render_error, render_page};

pub mod view;

pub fn router(state: crate::State) -> Router {
    Router::new()
        .route("/{slug}", get(handle_page))
        .with_state(state)
}

async fn handle_page(
    Path(slug): Path<String>,
    State(state): State<crate::State>,
) -> impl IntoResponse {
    match pages::get_page(slug, &state).await {
        Ok(page) => render_page(&page),
        Err(err) => render_error(&err),
    }
}
