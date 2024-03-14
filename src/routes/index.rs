use crate::{
    models::{todo_item::TodoItem, user::User},
    render_html::*,
    turso_helper::fetch_all,
    AppState,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_htmx::HxBoosted;
use libsql::params;
use minijinja::context;
use std::{sync::Arc, time::Instant};
use tower_sessions::Session;

pub async fn index(session: Session, State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Response {
    let session_user = session.get::<User>("user").await.unwrap();
    if session_user.is_none() {
        return render_html("home.html", context!(), boosted).unwrap().into_response();
    }
    let user = session_user.unwrap();
    let now = Instant::now();
    let todos: Vec<TodoItem> = fetch_all::<TodoItem>(
        &state.db_conn,
        "SELECT * FROM todos WHERE user_id = ?",
        params![user.id],
    )
    .await
    .unwrap();
    let elapsed = format!("{:?}", now.elapsed());

    let context = context!(todos, user, elapsed);
    return render_html("home_todos.html", context, boosted)
        .unwrap()
        .into_response();
}
