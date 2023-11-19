use crate::{models::*, render_html::*, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    Form,
};
use axum_htmx::{HxBoosted, HxRefresh};
use minijinja::context;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tower_cookies::cookie::time::Duration as CookieDuration;
use tower_cookies::{Cookie, Cookies};

pub async fn index(State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    let todos: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&state.db)
        .await
        .unwrap();
    let context = context!(todos,);
    return render_html("home.html", context, &state.jinja, boosted).unwrap();
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Form(form): Form<TodoItem>,
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

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

pub async fn login_post(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let mut errors = HashMap::new();

    let values = HashMap::from([("email", &form.email)]);

    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password, created_at FROM users WHERE email = ?",
        form.email
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();

    if user.is_none() {
        errors.insert("general", "Invalid email or password");
        return (
            HxRefresh(false),
            render_html("login.html", context! { errors, values }, &state.jinja, true).unwrap(),
        );
    }

    let user = user.unwrap();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    let password_matches = Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !password_matches {
        errors.insert("general", "Invalid email or password");
        return (
            HxRefresh(false),
            render_html("login.html", context! { errors, values }, &state.jinja, true).unwrap(),
        );
    }

    // set cookie
    // TODO handle sessions correctly
    let cookie = Cookie::build("user_id", user.id.to_string())
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(CookieDuration::days(30))
        .finish();
    cookies.add(cookie);

    return (HxRefresh(true), Html("".to_string()));
}

pub async fn login_get(State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    return render_html("login.html", context!(), &state.jinja, boosted).unwrap();
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    password: String,
    password2: String,
}

pub async fn register_post(State(state): State<Arc<AppState>>, Form(form): Form<RegisterForm>) -> Html<String> {
    let mut errors = HashMap::new();

    let values = HashMap::from([
        ("email", &form.email),
        ("password", &form.password),
        ("password2", &form.password2),
    ]);

    if form.password != form.password2 {
        errors.insert("password2", "Passwords do not match");
    }

    let email_exists = sqlx::query!("SELECT email FROM users WHERE email = ?", form.email)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if email_exists.is_some() {
        return render_block("login.html", "alert_success", context!(), &state.jinja).unwrap();
    }

    if !errors.is_empty() {
        // TODO Use error code 422
        return render_html("register.html", context! { errors, values }, &state.jinja, true).unwrap();
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(form.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        id: 0,
        email: form.email.clone(),
        password: hashed_password,
        created_at: 0,
    };

    let query_result = sqlx::query!(
        "INSERT INTO users (email,password) VALUES (?, ?)",
        user.email,
        user.password,
    )
    .execute(&state.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        errors.insert("general", "Could not create user");
        return render_html("register.html", context! { errors, values }, &state.jinja, true).unwrap();
    } else {
        return render_block("register.html", "alert_success", context!(), &state.jinja).unwrap();
    }
}

pub async fn register_get(State(state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    return render_html("register.html", context!(), &state.jinja, boosted).unwrap();
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

pub async fn handle_404(
    State(state): State<Arc<AppState>>,
    HxBoosted(boosted): HxBoosted,
) -> (StatusCode, Html<String>) {
    (
        StatusCode::OK,
        render_html("404.html", (), &state.jinja, boosted).unwrap(),
    )
}
