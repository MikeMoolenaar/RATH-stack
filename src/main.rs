use crate::filters::filters::date_string;
use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    BoxError, Router,
};
use dotenv::dotenv;
use minijinja::{path_loader, Environment};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::services::{ServeDir, ServeFile};

mod filters;
mod models;
mod routes;
mod serde_converters;

pub struct AppState {
    db: SqlitePool,
    jinja: Environment<'static>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to DB
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Sqlite::database_exists(&db_url)
        .await
        .expect("Database should exist, run `cargo sqlx database setup`");
    let db_pool = SqlitePool::connect(&db_url).await.expect("Database should connect");

    // Setup static file service
    let notfound_handler = ServeFile::new("static/404.html");
    let static_dir = ServeDir::new("static")
        .append_index_html_on_directories(true)
        .not_found_service(notfound_handler.clone());

    // Setup rate limiting
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(50)
            .use_headers()
            .finish()
            .unwrap(),
    );

    // Setup templating
    let mut jinja = Environment::new();
    jinja.set_loader(path_loader("templates"));
    jinja.add_filter("date_string", date_string);

    // Setup router
    let app = Router::new()
        .nest_service("/static", static_dir)
        .route("/", get(routes::index))
        .route("/todos", post(routes::create_todo))
        .route("/login", get(routes::login))
        .route("/json", get(routes::json))
        .route("/json-list", get(routes::json_list))
        .fallback_service(notfound_handler)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| async move { display_error(e) }))
                .layer(GovernorLayer {
                    config: Box::leak(governor_conf),
                }),
        )
        .with_state(Arc::new(AppState {
            db: db_pool.clone(),
            jinja,
        }));

    println!("Server is running at http://localhost:8080");

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
