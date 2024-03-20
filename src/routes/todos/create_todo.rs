use crate::{
    models::{todo_item::TodoItemRequest, user::User},
    render_html::render_html_str,
    AppState,
};
use axum::{extract::State, http::StatusCode, response::Html, Form};
use libsql::params;
use minijinja::context;
use std::sync::Arc;
use tower_sessions::Session;

pub async fn create_todo(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<TodoItemRequest>,
) -> Result<Html<String>, (StatusCode, String)> {
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

    return Ok(render_html_str(
        "Todo item '{{ title_clone }}' succesfuly added!",
        context! {
            title_clone
        },
    )
    .unwrap());
}
