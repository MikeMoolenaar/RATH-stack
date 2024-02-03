use crate::{
    models::{todo_item::TodoItem, user::User},
    render_html::*,
    AppState,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_htmx::HxBoosted;
use minijinja::context;
use std::sync::Arc;
use tower_sessions::Session;

pub async fn index(session: Session, State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Response {
    let session_user = session.get::<User>("user").unwrap();
    if session_user.is_none() {
        return render_html("home.html", context!(), boosted).unwrap().into_response();
    }
    let user = session_user.unwrap();
    let todos: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos WHERE user_id = ?", user.id)
        .fetch_all(&state.db)
        .await
        .unwrap();
    let context = context!(todos, user);
    return render_html("home_todos.html", context, boosted)
        .unwrap()
        .into_response();
}
