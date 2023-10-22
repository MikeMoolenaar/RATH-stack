use actix_files::Files;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use dotenv::dotenv;
use html_escape;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use std::str::FromStr;

#[derive(Serialize)]
struct Info {
    name: String,
    age: u32,
}

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello world!!!!");
}

#[get("/test")]
async fn test() -> impl Responder {
    return HttpResponse::Ok()
        .content_type("text/html")
        .body("<h4>Hello world!</h4>");
}

#[derive(Deserialize)]
struct TodoItemDto {
    #[serde(default)]
    id: i64,
    title: String,
    date: String,
}

#[derive(sqlx::FromRow)]
struct TodoItem {
    id: i64,
    title: String,
    date: i64,
}

#[post("/todos")]
async fn todos_post(
    web::Form(form): web::Form<TodoItemDto>,
    app: web::Data<AppState>,
) -> impl Responder {
    let date = NaiveDate::parse_from_str(form.date.as_str(), "%Y-%m-%d").unwrap();
    let date = date.and_hms_opt(0, 0, 0).unwrap().timestamp();
    let title = html_escape::encode_text(&form.title);
    let title_result = form.title.clone();

    let query_result = sqlx::query!("INSERT INTO todos (title,date) VALUES (?, ?)", title, date)
        .execute(&app.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return HttpResponse::InternalServerError().body("Unknown error!");
    }

    return HttpResponse::Ok().body(format!("Todo item '{}' succesfuly added", title_result));
}

#[get("/todos")]
async fn todos_get(app: web::Data<AppState>) -> impl Responder {
    let query_result: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&app.db)
        .await
        .unwrap();

    let mut str = String::from_str("<ul>").unwrap();

    for res in query_result {
        let date_formated: String = NaiveDateTime::from_timestamp_opt(res.date, 0)
            .unwrap()
            .format("%d-%m-%Y")
            .to_string();
        str += format!("<li>{} with date {}</li>", res.title, date_formated).as_str();
    }
    str += "</ul>";

    return HttpResponse::Ok().content_type("text/html").body(str);
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

#[get("/json")]
async fn json() -> web::Json<Info> {
    return web::Json(Info {
        name: String::from("Mike"),
        age: 24,
    });
}

#[get("/json-list")]
async fn json_list() -> web::Json<Vec<Info>> {
    let mut vec = Vec::new();

    for i in 0..5_000u32 {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        vec.push(Info { name: s, age: i });
    }
    return web::Json(vec);
}

pub struct AppState {
    db: SqlitePool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Setup DB
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Sqlite DB created!"),
            Err(error) => panic!("Could not create Sqlite DB due to error: {}", error),
        }
    } else {
        println!("Database already exists, moving on...");
    }

    // Setup table
    let db_pool = SqlitePool::connect(&db_url).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, title VARCHAR(250) NOT NULL, date INTEGER NOT NULL);").execute(&db_pool).await.unwrap();
    println!("Create user table result: {:?}", result);

    // Setup env
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Setup Actix api
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db: db_pool.clone(),
            }))
            .service(Files::new("/static", "src/static"))
            .service(hello)
            .service(test)
            .service(todos_get)
            .service(todos_post)
            .service(echo)
            .service(json)
            .service(json_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
