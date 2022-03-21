#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};

use std::{env, io};

mod constants;
mod like;
mod response;
mod schema;
mod tweet;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
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
