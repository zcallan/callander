use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpResponse,
};
use log::info;

use crate::db;
use crate::friends_ideas::actions;
use crate::friends_ideas::models;

#[get("/friends/{friend_id}/ideas")]
pub async fn find_all_ideas(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let friend_id = path.into_inner();

    let friend_ideas = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_friend_ideas(&mut conn, friend_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_ideas))
}

#[get("/friends/{friend_id}/ideas/{idea_id}")]
pub async fn find_one_idea(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (friend_id, idea_id) = path.into_inner();

    let friend: models::FriendsIdea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_friend_idea_by_id(&mut conn, idea_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[post("/friends/{friend_id}/ideas")]
pub async fn create_idea(
    path: web::Path<String>,
    new_friend_idea: Json<models::NewFriendsIdea>,
) -> Result<HttpResponse, Error> {
    let friend_id = path.into_inner();

    let friend_idea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create_friend_idea(&mut conn, friend_id, &new_friend_idea)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_idea))
}

#[post("/friends/{friend_id}/ideas/{idea_id}")]
pub async fn update_idea(
    path: web::Path<(String, String)>,
    update_friend_idea: Json<models::UpdateFriendsIdea>,
) -> Result<HttpResponse, Error> {
    let (friend_id, idea_id) = path.into_inner();

    let friend_idea = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::update_friend_idea(&mut conn, idea_id, &update_friend_idea)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_idea))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all_ideas);
    config.service(find_one_idea);
    config.service(create_idea);
    config.service(update_idea);
}
