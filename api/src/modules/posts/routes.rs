use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpResponse,
};

use crate::posts::models::{NewPost, Post, PostsFindAllQuery};
use crate::{auth::middleware::JwtMiddleware, posts::actions};
use crate::{db, utils::pagination::Paginated};

#[get("/posts")]
pub async fn find_all(
    info: web::Query<PostsFindAllQuery>,
    jwt: JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;

    let options = PostsFindAllQuery {
        page: info.page,
        per_page: info.per_page,
    };

    let posts: Paginated<Vec<Post>> = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_posts(&mut conn, user_id, &options)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
pub async fn find_one(path: web::Path<String>, jwt: JwtMiddleware) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let id = path.into_inner();

    let post: Post = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_post_by_id(&mut conn, user_id, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
pub async fn create(new_post: Json<NewPost>, jwt: JwtMiddleware) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;

    let post: Post = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create_post(&mut conn, user_id, &new_post)
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
