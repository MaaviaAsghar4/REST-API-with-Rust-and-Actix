#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use std::{env, io};

mod constants;
mod like;
mod response;
mod schema;
mod tweet;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // setting uo database connection
    let database_url = env::var("DATABASE_URL").expect("Invalid URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // set up db pool
            .app_data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::list)
            .service(tweet::get)
            .service(tweet::create)
            .service(tweet::delete)
            .service(like::list)
            .service(like::plus_one)
            .service(like::minus_one)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
