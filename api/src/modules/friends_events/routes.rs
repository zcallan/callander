use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use serde::Deserialize;

use super::actions;
use super::models;

use crate::{auth::middleware::JwtMiddleware, db};

#[derive(Debug, Deserialize)]
pub struct FindAllQueryParams {
    friend_id: String,
}

#[get("/friend-events/{id}")]
pub async fn find_one(path: web::Path<String>, jwt: JwtMiddleware) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let id = path.into_inner();

    let friend: models::FriendsEvent = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_by_id(&mut conn, user_id, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[get("/friend-events")]
pub async fn find_all(req: HttpRequest, jwt: JwtMiddleware) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;

    let params = web::Query::<FindAllQueryParams>::from_query(req.query_string())
        .expect("Expected friend_id");
    let friend_id = params.friend_id.clone();

    let friend_events = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all(&mut conn, user_id, friend_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_events))
}

#[post("/friend-events")]
pub async fn create(
    new_friend_event: Json<models::NewFriendsEvent>,
    jwt: JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;

    let friend_event = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create(&mut conn, user_id, &new_friend_event)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_event))
}

#[post("/friend-events/{id}")]
pub async fn update(
    path: web::Path<String>,
    update_friend_event: Json<models::UpdateFriendsEvent>,
    jwt: JwtMiddleware,
) -> Result<HttpResponse, Error> {
    let user_id = jwt.user_id;
    let id = path.into_inner();

    let friend_event = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::update(&mut conn, user_id, id, &update_friend_event)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_event))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);
}
