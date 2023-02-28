use actix_web::{
    get, post,
    web::{self, Data, Json, Path, ServiceConfig},
    Error, HttpResponse,
};

use crate::db;
use crate::posts::actions;
use crate::posts::models::{NewPost, Post};

#[get("/posts")]
pub async fn find_all() -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_posts(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[post("/posts")]
pub async fn create(new_post: Json<NewPost>) -> Result<HttpResponse, Error> {
    let post = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create_post(&mut conn, &new_post)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    // config.service(find_one);
    config.service(create);
    // config.service(update);
    // config.service(delete);
}
