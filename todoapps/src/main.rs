use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use askama_actix::TemplateToResponse;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse {
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    hello.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
