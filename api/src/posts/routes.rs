use actix_web::{
    get, post,
    web::{self, Data, Json, Path, ServiceConfig},
    Error, HttpResponse,
};
use serde::Deserialize;

use crate::db;
use crate::posts::actions;
use crate::posts::models::{NewPost, Post};

#[derive(Deserialize)]
pub struct FindAllQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[get("/posts")]
pub async fn find_all(info: web::Query<FindAllQuery>) -> Result<HttpResponse, Error> {
    let limit = info.limit.unwrap_or(10);
    let offset = info.offset.unwrap_or(0);

    let posts: Vec<Post> = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_posts(&mut conn, limit, offset)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
pub async fn find_one(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let post: Post = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_post_by_id(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
pub async fn create(new_post: Json<NewPost>) -> Result<HttpResponse, Error> {
    let post: Post = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create_post(&mut conn, &new_post)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    // config.service(update);
    // config.service(delete);
}
