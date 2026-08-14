use axum::extract::{Path, State};
use axum::response::IntoResponse;

use crate::model::pages::{self};
use crate::routes::views::{render_error, render_page};

pub async fn handle_page(
    Path(slug): Path<String>,
    State(state): State<crate::State>,
) -> impl IntoResponse {
    match pages::get_page(slug, &state).await {
        Ok(page) => render_page(&page),
        Err(err) => render_error(&err),
    }
}
