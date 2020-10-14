extern crate dotenv;
extern crate env_logger;
extern crate juniper;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate actix_rt;
extern crate actix_web;

mod api;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};

use api::db::get_pool;
use api::endpoints::graphql_endpoints;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();
    let pool = get_pool();

    // Build and start the server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
