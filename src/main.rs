use crate::{
    filters::filters::date_string,
};
use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, Request, StatusCode},
    routing::{get, post},
    BoxError, Router,
};
use dotenv::dotenv;
use minijinja::{path_loader, Environment};
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tower::{ServiceBuilder};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;
use tower_sessions::{
    cookie::SameSite, session_store::ExpiredDeletion, sqlx::SqlitePool, Expiry, SessionManagerLayer, SqliteStore,
};

mod filters;
mod models;
mod render_html;
mod routes;
mod serde_converters;

pub struct AppState {
    db: SqlitePool,
    jinja: Environment<'static>,
}

fn not_htmx_predicate<Body>(req: &Request<Body>) -> bool {
    !req.headers().contains_key("hx-request")
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

    let session_store = SqliteStore::new(db_pool.clone());
    session_store.migrate().await.expect("Could not migrate session store");
    tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(60)),
    );

    let session_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async { StatusCode::BAD_REQUEST }))
        .layer(
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
    // TODO use the tower layer https://github.com/tokio-rs/axum/discussions/987#discussioncomment-2678595
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
        .nest_service("/static/dist", static_dit_dist_service)
        .nest_service("/static", static_dir)
        .fallback(routes::handle_static_404)
        .route("/favicon.ico", get(routes::handle_static_404))
        .route("/", get(routes::index))
        .route("/todos", post(routes::create_todo))
        .route("/login", get(routes::login_get).post(routes::login_post))
        .route("/logout", post(routes::logout))
        .route("/register", get(routes::register_get).post(routes::register_post))
        .route("/json", get(routes::json))
        .route("/json-list", get(routes::json_list))
        .layer(session_layer)
        .fallback(routes::handle_page_404)
        .layer(
            LiveReloadLayer::new()
                .request_predicate(not_htmx_predicate)
                .reload_interval(Duration::from_millis(100)),
        )
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

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
