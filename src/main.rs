use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Info {
    todo: String,
}

#[get("/")]
async fn hello() -> Result<NamedFile> {
    let path: PathBuf = "./public/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[post("/echo")]
async fn echo(req_body: web::Json<Info>) -> impl Responder {
    let response = format!("<h1>{}</h1>", req_body.todo);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
