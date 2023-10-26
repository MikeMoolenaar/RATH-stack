use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_files::Files;
use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use std::time::Duration;

mod error_handlers;
mod models;
mod routes;
mod serde_converters;

pub struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup env
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Connect to DB
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Sqlite::database_exists(&db_url)
        .await
        .expect("Database should exist, run `cargo sqlx database setup`");
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Database should connect");

    let rate_limiter_middleware = InMemoryBackend::builder().build();

    println!("Web server running at http://localhost:8080");

    // Setup Actix api
    HttpServer::new(move || {
        // Assign a limit of 5 requests per minute per client ip address
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 180)
            .real_ip_key()
            .build();
        let rate_limiter_middleware = RateLimiter::builder(rate_limiter_middleware.clone(), input)
            .add_headers()
            .build();

        App::new()
            .wrap(rate_limiter_middleware)
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, error_handlers::error_500)
                    .handler(StatusCode::TOO_MANY_REQUESTS, error_handlers::error_429),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db: db_pool.clone(),
            }))
            .service(Files::new("/static", "src/static"))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
