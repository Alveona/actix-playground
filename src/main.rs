#[macro_use] extern crate log;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
mod constants;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().json(Response{
        message: "Hello, Actix!".to_string()
    })
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();


    let connection_string = format!("{}:{}", constants::HOST, constants::PORT);
    println!("Running web server on {}", connection_string);


    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind(connection_string)?
    .run()
    .await
}