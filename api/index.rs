use actix_web::http::header::ContentType;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    status: String
}

#[get("/")]
async fn hello() -> impl Responder {
    let response = Response { status: "OK".into() };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&response).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
