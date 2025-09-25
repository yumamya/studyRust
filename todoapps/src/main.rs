use actix_web::{get, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Row, SqlitePool};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplete {
    tasks: Vec<String>,
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse {
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    hello.to_response()
}

#[get("/")]
async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    let rows = sqlx::query("SELECT task FROM tasks;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let tasks: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("task"))
        .collect();
    let todo = TodoTemplete { tasks };
    todo.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE tasks (task TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('タスクq１')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('タスクq２')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('タスクq３')")
        .execute(&pool)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(todo)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
