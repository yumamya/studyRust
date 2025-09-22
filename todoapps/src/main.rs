use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> String {
    format!("Hello {name}!!")
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
