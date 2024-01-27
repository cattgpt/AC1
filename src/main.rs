mod models;
mod config;

use actix_web::web;
use actix_web::{HttpServer, App, Responder};
use actix_web::HttpResponse;
use crate::models::Status;
use dotenv::dotenv;
use std::io;
use crate::config::Config;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status {status: "UP".to_string()})

}

// specify main function so it runs in the Actix run time (rt)
#[actix_rt::main]
async fn main()-> io::Result<()> {
    // load server configs
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    
    // print and start the server
    println!("Starting serer at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
       App::new()
         .app_data(pool.clone())
         .route("/", web::get().to(status))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
    
}
