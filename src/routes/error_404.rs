use crate::render_html::*;
use axum::{http::StatusCode, response::Html};
use axum_htmx::HxBoosted;

pub async fn handle_static_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn handle_page_404(HxBoosted(boosted): HxBoosted) -> (StatusCode, Html<String>) {
    (StatusCode::NOT_FOUND, render_html("404.html", (), boosted).unwrap())
}
