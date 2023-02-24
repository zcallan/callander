mod api;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{http, web::Data, App, HttpServer};

use api::user_api::{create_user, get_user, get_all_users};
use api::friend_api::{create_friend, get_friend, get_all_friends};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
            )
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(get_all_users)
            .service(create_friend)
            .service(get_friend)
            .service(get_all_friends)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
