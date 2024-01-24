use crate::{render_html::*, AppState};
use axum::{extract::State, http::StatusCode, response::Html};
use axum_htmx::HxBoosted;
use std::sync::Arc;

pub async fn handle_static_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn handle_page_404(
    State(state): State<Arc<AppState>>,
    HxBoosted(boosted): HxBoosted,
) -> (StatusCode, Html<String>) {
    (
        StatusCode::NOT_FOUND,
        render_html("404.html", (), &state.jinja, boosted).unwrap(),
    )
}
