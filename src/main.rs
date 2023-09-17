use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Info {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let contents = fs::read_to_string("./public/index.html").unwrap();
    HttpResponse::Ok().body(contents)
}

#[post("/echo")]
async fn echo(req_body: web::Json<Info>) -> impl Responder {
    let response = format!("<h1>Hello, my name is {}</h1>", req_body.name);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
