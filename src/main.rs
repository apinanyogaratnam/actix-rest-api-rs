use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize)]
struct Greeting {
    message: &'static str,
}

async fn greeting() -> impl Responder {
    web::Json(Greeting { message: "Hello world!" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/greeting", web::get().to(greeting))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
