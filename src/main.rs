use crate::{filters::*, render_html::SHARED_JINJA_ENV, routes::error_404::*};
use axum::{
    body::Body,
    http::{header, HeaderValue, Request}, Router,
};
use dotenv::dotenv;
use minijinja::{path_loader, Environment};
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Pool, Postgres};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;
use tower_sessions::{cookie::SameSite, session_store::ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

mod filters;
mod models;
mod render_html;
mod routes;
mod serde_converters;

pub struct AppState {
    db: Pool<Postgres>,
}

fn not_htmx_predicate(req: &Request<Body>) -> bool {
    !req.headers().contains_key("hx-request")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Connect to DB
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Postgres::database_exists(&db_url)
        .await
        .expect("Database should exist, run `cargo sqlx database setup`");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    let session_store = PostgresStore::new(db_pool.clone());
    session_store.migrate().await.expect("Could not migrate session store");
    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(60)),
    );

    let session_layer = ServiceBuilder::new().layer(
        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_http_only(true)
            .with_same_site(SameSite::Strict)
            .with_expiry(Expiry::OnInactivity(time::Duration::days(7))),
    );

    // Setup templating
    let mut jinja = Environment::new();
    jinja.set_loader(path_loader("templates"));
    jinja.add_filter("date_string", date_string);
    let _ = SHARED_JINJA_ENV.set(jinja.clone());

    // Setup static file service
    let static_dir_dist = ServeDir::new("static/dist");
    let static_dit_dist_service = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ))
        .service(static_dir_dist);
    let static_dir = ServeDir::new("static").append_index_html_on_directories(true);

    // Setup rate limiting
    // replenish one element every 500 milliseconds, up to 30 requests per ip
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(500)
            .burst_size(30)
            .use_headers()
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        governor_limiter.retain_recent();
    });
    let governor_layer = ServiceBuilder::new().layer(GovernorLayer {
        config: Box::leak(governor_conf),
    });

    // Setup router
    let mut app = Router::new()
        .nest_service("/static/dist", static_dit_dist_service)
        .nest_service("/static", static_dir)
        .fallback(handle_static_404)
        .merge(routes::router())
        .layer(governor_layer)
        .layer(session_layer)
        .fallback(handle_page_404)
        .with_state(Arc::new(AppState { db: db_pool.clone() }));

    if cfg!(debug_assertions) {
        app = app.layer(
            LiveReloadLayer::new()
                .request_predicate(not_htmx_predicate)
                .reload_interval(Duration::from_millis(100)),
        )
    }

    println!("Server is running at http://localhost:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
