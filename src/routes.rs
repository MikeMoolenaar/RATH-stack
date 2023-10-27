use crate::{models::*, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use leptos::ssr::render_to_string;
use leptos::{view, CollectView, IntoView};
use rand::{distributions::Alphanumeric, Rng};

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello world!<br> You want to go to <a href=\"/static/index.html\">static/index.html</a> right?");
}

#[get("/test")]
async fn test() -> impl Responder {
    let html = render_to_string(|| {
        view! {
            <h1>Hi</h1>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(html.to_string());
}

#[post("/todos")]
async fn todos_post(
    web::Form(form): web::Form<TodoItem>,
    app: web::Data<AppState>,
) -> impl Responder {
    let title_clone = form.title.clone();

    let query_result = sqlx::query!(
        "INSERT INTO todos (title,date) VALUES (?, ?)",
        form.title,
        form.date
    )
    .execute(&app.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        println!("Could not execute insert due to error: {}", err);
        return HttpResponse::InternalServerError().body("Unknown error!");
    }

    return HttpResponse::Ok().body(format!("Todo item '{}' succesfuly added", title_clone));
}

#[get("/todos")]
async fn todos_get(app: web::Data<AppState>) -> impl Responder {
    let query_result: Vec<TodoItem> = sqlx::query_as!(TodoItem, "SELECT * FROM todos")
        .fetch_all(&app.db)
        .await
        .unwrap();

    let items = query_result
        .iter()
        .map(|res| {
            let date_formatted: String = NaiveDateTime::from_timestamp_opt(res.date, 0)
                .unwrap()
                .format("%d-%m-%Y")
                .to_string();
            return view! {
                <li>{format!("{} with date {}", res.title, date_formatted)}</li>
            };
        })
        .collect_view();

    let html = render_to_string(|| {
        view! {
            <ul class="list-disc list-inside">
                {items}
            </ul>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(html.to_string());
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

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(hello)
        .service(test)
        .service(todos_get)
        .service(todos_post)
        .service(echo)
        .service(json)
        .service(json_list);
}
