use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    BoxError, Router,
};
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::services::{ServeDir, ServeFile};

mod models;
mod routes;
mod serde_converters;

pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to DB
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Sqlite::database_exists(&db_url)
        .await
        .expect("Database should exist, run `cargo sqlx database setup`");
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Database should connect");

    // Setup static file service
    let static_dir = ServeDir::new("src/static")
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new("src/static/404.html"));

    // Setup rate limiting
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(50)
            .use_headers()
            .finish()
            .unwrap(),
    );

    // Setup router
    let app = Router::new()
        .nest_service("/static", static_dir)
        .route("/", get(routes::index))
        .route("/test", get(routes::test))
        .route("/todos", post(routes::create_todo).get(routes::get_todos))
        .route("/json", get(routes::json))
        .route("/json-list", get(routes::json_list))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|e: BoxError| async move {
                    display_error(e)
                }))
                .layer(GovernorLayer {
                    config: Box::leak(governor_conf),
                }),
        )
        .with_state(Arc::new(AppState {
            db: db_pool.clone(),
        }));

    println!("Server is running at http://localhost:8080");

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
