use crate::models::{Status,CreateTodoList,ResultResponse};
use crate::db;
use std::io::ErrorKind::Other;
use deadpool_postgres::{Pool,Client};
use actix_web::{web,Responder,HttpResponse};

pub async fn status() -> impl Responder{
    web::HttpResponse::Ok()
     .json(Status {status:"UP".to_string()})
 }

 pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder{

    let client: Client = 
        db_pool.get().await.expect("Error to connect with the database");

    let result = db::get_todos(&client).await;

    match result{
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }

 }

/*  pub async fn items(db_pool: web::Data<Pool>,path: web::Path<i16>) -> impl Responder{

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    HttpResponse::Ok().body("Hello world!")
   
    /* 


    let result = db::get_items(&client, list_id ).await;


    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    } */

 } */

 pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error to connect with the database");

    let result = db::create_todo(&client, json.title.clone()).await;


    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder{

    let client: Client = 
        db_pool.get().await.expect("Error to connect with the database");

    let result = db::check_item(&client, path.0.0, path.1 ).await;


    match result{
        Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }

 }