use actix_web::{get, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;

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
async fn todo() -> HttpResponse {
    let tasks = vec![
        "タスク１".to_string(),
        "タスク２".to_string(),
        "タスク３".to_string(),
    ];
    let todo = TodoTemplete { tasks };
    todo.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
