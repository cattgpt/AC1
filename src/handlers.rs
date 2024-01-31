use actix_web::{web,HttpResponse, Responder};
use crate::models::{Status, CreateTodoList};
use crate::db;
use deadpool_postgres::{Pool, Client};

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client =
       db_pool.get().await.expect("Error connecting to the data base");
    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

// gets value from path
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client =
       db_pool.get().await.expect("Error connecting to the data base");
    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

// gets value from body as JSON
pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {
    let client: Client =
       db_pool.get().await.expect("Error connecting to the data base");
       // get a copy of string using clone()
    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder {
    let client: Client =
       db_pool.get().await.expect("Error connecting to the data base");
    let result = db::check_item(&client, path.0, path.1).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}