use crate::models::*;
use crate::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, response::Json, Form};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;
use askama::Template;
use crate::filters::*;

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate {
    todos: Vec<TodoItem>,
}

pub async fn index(State(data): State<Arc<AppState>>) -> IndexTemplate {
    let todos: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&data.db)
        .await
        .unwrap();
    return IndexTemplate { todos };
}


pub async fn create_todo(
    State(data): State<Arc<AppState>>,
    Form(form): Form<TodoItem>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let title_clone = form.title.clone();

    let query_result = sqlx::query!(
        "INSERT INTO todos (title,date) VALUES (?, ?)",
        form.title,
        form.date
    )
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Unknown error"),
        ));
    }

    return Ok(format!("Todo item '{}' succesfuly added", title_clone));
}
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    // TODO
}
pub async fn login() -> LoginTemplate {
    return LoginTemplate {};
}

pub async fn json() -> Json<Info> {
    return Json(Info {
        name: String::from("Mike"),
        age: 24,
    });
}

pub async fn json_list() -> Json<Vec<Info>> {
    let mut vec = Vec::new();

    for i in 0..5_000u32 {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        vec.push(Info { name: s, age: i });
    }
    return Json(vec);
}