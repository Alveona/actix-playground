#[macro_use] extern crate log;
#[allow(dead_code)]
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{ser, de, Deserialize, Deserializer, Serialize};
mod constants;
use qstring::QString;
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Deserialize)]
struct QueryStruct{
    #[serde(default, deserialize_with = "deserialize_some")]
    text: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    id: Option<u64>
}

fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

async fn index(web::Query(query): web::Query<QueryStruct>) -> impl Responder {
    if let Some(_text) = query.text {
        println!("{:?}", _text);
    }
    
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
            .route("/hw", web::get().to(index2))
    })
    .bind(connection_string)?
    .run()
    .await
}