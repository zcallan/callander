use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpResponse,
};

use crate::posts::actions;
use crate::posts::models::{NewPost, Post, PostsFindAllQuery};
use crate::{db, utils::pagination::Paginated};

#[get("/posts")]
pub async fn find_all(info: web::Query<PostsFindAllQuery>) -> Result<HttpResponse, Error> {
    let options = PostsFindAllQuery {
        page: info.page,
        per_page: info.per_page,
    };

    let posts: Paginated<Vec<Post>> = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_posts(&mut conn, &options)
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
