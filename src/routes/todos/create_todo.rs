use crate::{
    models::{todo_item::TodoItemRequest, user::User},
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use libsql::params;
use std::sync::Arc;
use tower_sessions::Session;

pub async fn create_todo(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<TodoItemRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let title_clone = form.title.clone();
    let user = session.get::<User>("user").await.unwrap().unwrap();

    let query_result = state
        .db_conn
        .execute(
            "INSERT INTO todos (title,date,user_id) VALUES (?, ?, ?)",
            params![form.title, form.date, user.id],
        )
        .await;

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Unknown error")));
    }

    return Ok(format!("Todo item '{}' succesfuly added", title_clone));
}
