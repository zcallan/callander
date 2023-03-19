#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod auth;
mod config;
mod db;
mod error_handler;
mod modules;
mod schema;
mod utils;

use actix_web::{http, middleware::Logger, web, App, HttpServer};
use config::cors::with_cors;
use log::info;
use modules::{friends, friends_events, friends_ideas, posts};
use utils::env::{get_host_port, init_env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();

    db::init();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (host, port) = get_host_port();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(with_cors())
            .configure(friends::routes::init_routes)
            .configure(friends_events::routes::init_routes)
            .configure(friends_ideas::routes::init_routes)
            .configure(posts::routes::init_routes)
    })
    .bind((host, port))?
    .run();

    info!("Server started on port {}", port);

    server.await
}
