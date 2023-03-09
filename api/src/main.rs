#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;

mod db;
mod error_handler;
mod modules;
mod schema;
mod utils;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, App, HttpServer};
use log::info;
use modules::{friends, friends_events, friends_ideas, posts, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .configure(friends::routes::init_routes)
            .configure(friends_events::routes::init_routes)
            .configure(friends_ideas::routes::init_routes)
            .configure(posts::routes::init_routes)
            .configure(users::routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    info!("Server started on port 8080");

    server.await
}
