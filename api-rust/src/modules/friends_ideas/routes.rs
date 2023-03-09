use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use serde::Deserialize;

use crate::db;
use crate::friends_ideas::actions;
use crate::friends_ideas::models;

#[derive(Debug, Deserialize)]
pub struct FindAllQueryParams {
    friend_id: String,
}

#[get("/friend-ideas/{id}")]
pub async fn find_one(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend: models::FriendsIdea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_by_id(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[get("/friend-ideas")]
pub async fn find_all(req: HttpRequest) -> Result<HttpResponse, Error> {
    let params = web::Query::<FindAllQueryParams>::from_query(req.query_string())
        .expect("Expected friend_id");
    let friend_id = params.friend_id.clone();

    let friend_ideas = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all(&mut conn, friend_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_ideas))
}

#[post("/friend-ideas")]
pub async fn create(new_friend_idea: Json<models::NewFriendsIdea>) -> Result<HttpResponse, Error> {
    let friend_idea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create(&mut conn, &new_friend_idea)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_idea))
}

#[post("/friend-ideas/{id}")]
pub async fn update(
    path: web::Path<String>,
    update_friend_idea: Json<models::UpdateFriendsIdea>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend_idea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::update(&mut conn, id, &update_friend_idea)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_idea))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);
}
