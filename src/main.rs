#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use blogrs::{create_connection_pool, find_all_posts};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub mod models;
pub mod schema;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("COULD NOT LOAD THE DB POOL");

    match web::block(move || find_all_posts(&conn)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting webserver...");

    let pool = create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}
