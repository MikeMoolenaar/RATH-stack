use crate::{
    routes::{
        auth::{
            login::*,
            logout::*,
            register::{register::*, register_email_validate::*},
        },
        error_404::*,
        index::index,
        todos::create_todo::*,
    },
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub mod error_404;
pub mod index;
pub mod auth {
    pub mod login;
    pub mod logout;
    pub mod register {
        pub mod register;
        pub mod register_email_validate;
    }
}
pub mod todos {
    pub mod create_todo;
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/favicon.ico", get(handle_static_404))
        .route("/", get(index))
        .route("/todos", post(create_todo))
        .route("/login", get(login_get).post(login_post))
        .route("/logout", post(logout))
        .route("/register", get(register_get).post(register_post))
        .route("/register/check", get(register_email_validate))
}
