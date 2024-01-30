mod models;
mod config;
mod handlers;
mod db;

use actix_web::web;
use actix_web::web::Data;
use actix_web::{HttpServer, App};
use dotenv::dotenv;
use std::io;
use crate::config::Config;
use tokio_postgres::NoTls;
use crate::handlers::*;


// specify main function so it runs in the Actix run time (rt)
#[actix_rt::main]
async fn main()-> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // load server configs
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    
    // print and start the server
    println!("Starting serer at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
       App::new()
         .app_data(Data::new(pool.clone()))
         .route("/", web::get().to(status))
         .route("/todos{_:/?}", web::get().to(get_todos))
         .route("/todos/{list_id}/items", web::get().to(get_items))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
    
}
