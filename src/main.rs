mod config;
mod models;
mod handlers;
mod db;

use crate::handlers::*;
use actix_web::{middleware, web, get, App, HttpServer, Responder,HttpResponse};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool,Client};


/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/todos/{list_id}/items")] // <- define path parameters /todos/{list_id}/items{_:/?}
async fn mytest(db_pool: web::Data<Pool>,web::Path(list_id): web::Path<i32>) -> impl Responder {

    let client: Client = 
    db_pool.get().await.expect("Error to connect with the database");

    let result = db::get_items(&client, list_id ).await;


    match result{
        Ok(items) => HttpResponse::Ok().json(items),
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
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}",web::get().to(get_todos))
            .route("/todos{_:/?}",web::post().to(create_todo))
            //.route("/todos/{list_id}/items{_:/?}",web::get().to(items))
            .route("/todos/{list_id}/items{item_id}{_:/?}",web::put().to(check_item))
            .service(mytest)
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}