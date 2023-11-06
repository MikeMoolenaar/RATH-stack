use crate::{models::*, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    Form,
};
use axum_htmx::HxBoosted;
use minijinja::{context, Environment};
use rand::{distributions::Alphanumeric, Rng};
use serde::ser::Serialize;
use std::sync::Arc;

pub fn render_html<S: Serialize>(template_name: &str, context: S, jinja_env: &Environment, boosted: bool) -> Option<Html<String>> {
    // TODO Replace unwraps with better error handling
    // TODO Use global jinja_env so we don't have to always pass it
    //   https://github.com/photino/zino/blob/main/zino-core/src/view/minijinja.rs
    let tpl = jinja_env.get_template(template_name).unwrap();

    if boosted {
        let title = tpl.eval_to_state(context!()).unwrap().render_block("title").unwrap();
        let content = tpl.eval_to_state(context).unwrap().render_block("body").unwrap();
        let combined = format!("<title>{}</title>\n{}", title, content);
        return Some(Html(combined));

    } else {
        let content = tpl.render(context).unwrap();
        return Some(Html(content));
    }
}

pub async fn index(State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    let todos: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap();
    let context = context!(
    todos,
    );
    return render_html("home.html", context, &state.jinja, boosted).unwrap();
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Form(form): Form<TodoItem>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let title_clone = form.title.clone();

    let query_result = sqlx::query!("INSERT INTO todos (title,date) VALUES (?, ?)", form.title, form.date)
        .execute(&state.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Unknown error")));
    }

    return Ok(format!("Todo item '{}' succesfuly added", title_clone));
}

pub async fn login(State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    return render_html("login.html", context!(), &state.jinja, boosted).unwrap();
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
