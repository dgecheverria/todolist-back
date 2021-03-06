mod config;
mod models;
mod handlers;
mod db;

use crate::models::{ResultResponse,CreateTodoItem};
use crate::handlers::*;
use actix_web::{web, get, put, post, App, HttpServer, Responder,HttpResponse};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool,Client};
use std::io::ErrorKind::Other;
use actix_cors::Cors;

//Services

#[get("/todos/{list_id}/items/{item_id}")] // <- define path 
async fn get_item_by_id(db_pool: web::Data<Pool>,web::Path((list_id, item_id)): web::Path<(i32,i32)>) -> impl Responder {

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    let result = db::get_item_by_id(&client, list_id, item_id).await;

    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    } 
}


#[get("/todos/{list_id}/complete")] // <- define path 
async fn get_complete_item_by_id(db_pool: web::Data<Pool>,web::Path(list_id): web::Path<i32>) -> impl Responder {

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    let result = db::get_items_complete(&client, list_id).await;

    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    } 
}


#[get("/todos/{list_id}/items")] // <- define path 
async fn get_items_by_list(db_pool: web::Data<Pool>,web::Path(list_id): web::Path<i32>) -> impl Responder {

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    let result = db::get_items(&client, list_id ).await;

    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    } 
}

#[post("/todos/{list_id}/item")] // <- define path 
async fn create_item(db_pool: web::Data<Pool>,web::Path(list_id): web::Path<i32>,json: web::Json<CreateTodoItem>) -> impl Responder {

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    let result = db::create_item(&client, list_id ,json.title.clone()).await;

    match result{
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    } 
}

#[put("/todos/{list_id}/item/{item_id}")] // <- define path 
async fn check_item_list(db_pool: web::Data<Pool>,web::Path((list_id,item_id)): web::Path<(i32,i32)>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error to connect with the database");

    let result = db::check_item(&client, list_id, item_id ).await;

    match result{
        Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


#[actix_web::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Server Up: http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new( move || {
        App::new()
            .data(pool.clone())
            .wrap(Cors::new().allowed_origin("http://localhost:4200").finish())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}",web::get().to(get_todos))//List all list of task
            .route("/todos{_:/?}",web::post().to(create_todo))// Create a new list
            .service(create_item)//Create item
            .service(get_items_by_list)//Get all items by List
            .service(check_item_list)//Check Task
            .service(get_item_by_id)//Get a item by id
            .service(get_complete_item_by_id)

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}