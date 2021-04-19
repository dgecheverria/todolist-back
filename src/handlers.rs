use crate::models::{Status,CreateTodoList};
use crate::db;
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

 pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder{
    let client: Client = 
        db_pool.get().await.expect("Error to connect with the database");

    let result = db::create_todo(&client, json.title.clone()).await;


    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

