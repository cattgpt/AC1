mod models;

use actix_web::{HttpServer, App, Responder};
use std::io;
use actix_web::web;
use actix_web::HttpResponse;

use crate::models::Status;

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})

}

// specify main function so it runs in the Actix run time (rt)
#[actix_rt::main]
async fn main()-> io::Result<()> {
    println!("Starting serer at 127.0.0.1:8080");
    HttpServer::new(|| {
       App::new()
         .route("/", web::get().to(status))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}
