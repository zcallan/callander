mod db;
mod friend;
mod user;

use actix_cors::Cors;
use actix_web::{http, web::Data, App, HttpServer};

use db::Db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Db::init().await;
    let db_data = Data::new(db.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(db_data)
            .configure(user::routes::init_routes)
            .configure(friend::routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
