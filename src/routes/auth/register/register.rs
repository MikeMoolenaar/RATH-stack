use crate::{models::user::User, render_html::*, turso_helper::*, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, response::Html, Form};
use axum_htmx::HxBoosted;
use libsql::params;
use minijinja::context;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

pub async fn register_get(HxBoosted(boosted): HxBoosted) -> Html<String> {
    return render_html("register.html", context!(), boosted).unwrap();
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    password: String,
    password2: String,
}

pub async fn register_post(
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> (StatusCode, Html<String>) {
    let mut errors = HashMap::new();

    let values = HashMap::from([
        ("email", &form.email),
        ("password", &form.password),
        ("password2", &form.password2),
    ]);

    if form.password != form.password2 {
        errors.insert("password2", "Passwords do not match");
    }

    let count = count(
        &state.db_conn,
        "SELECT count(*) FROM users WHERE email = $1",
        params![form.email.clone()],
    )
    .await
    .unwrap();

    if count > 0 {
        errors.insert("email", "Email already exists");
    }

    if !errors.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            render_html("register.html", context! { errors, values }, true).unwrap(),
        );
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

    let query_result = state
        .db_conn
        .execute(
            "INSERT INTO users (email,password,created_at) VALUES (?, ?, ?)",
            params![user.email, user.password, chrono::Utc::now().timestamp()],
        )
        .await;

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        errors.insert("general", "Could not create user");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            render_html("register.html", context! { errors, values }, true).unwrap(),
        );
    } else {
        return (
            StatusCode::OK,
            render_block("register.html", "alert_success", context!()).unwrap(),
        );
    }
}
