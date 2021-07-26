#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;

use actix_web::{web, get, post,  middleware, App, HttpResponse, HttpServer, Responder};

use std::{env, io};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod schema;
mod geo;
mod ws;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(ws::index)))
            .service(geo::create)
            //.route("/ws/", web::get().to(ws::index))
            .service(echo)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}