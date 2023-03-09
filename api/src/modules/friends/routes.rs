use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self, Json, Query, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};

use crate::friends::actions;
use crate::friends::models;
use crate::{db, modules::friends_ideas};

#[get("/friends")]
pub async fn find_all(req: HttpRequest) -> Result<HttpResponse, Error> {
    let params = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    let default_sort_by = &String::from("created_at");
    let default_sort_order = &String::from("desc");

    let sort_by = params.get("sort_by").unwrap_or(default_sort_by);
    let sort_order = params.get("sort_order").unwrap_or(default_sort_order);

    let sort = models::FriendsFindAllSort {
        sort_by: sort_by.to_string(),
        sort_dir: sort_order.to_string(),
    };

    let mut conn = db::connection().expect("Error");
    let friends = actions::find_all(&mut conn, sort).expect("Failed to get friends");

    Ok(HttpResponse::Ok().json(friends))
}

#[get("/friends/{id}")]
pub async fn find_one(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend: models::Friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_by_id(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[post("/friends")]
pub async fn create(new_friend: Json<models::NewFriend>) -> Result<HttpResponse, Error> {
    let friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create(&mut conn, &new_friend)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[post("/friends/{id}")]
pub async fn update(
    path: web::Path<String>,
    update_friend: Json<models::UpdateFriend>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::update(&mut conn, id, &update_friend)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[get("/friends/{id}/ideas")]
pub async fn find_all_friend_ideas(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let friend_id = path.into_inner();

    let friend_ideas = web::block(move || {
        let mut conn = db::connection().expect("Error");
        friends_ideas::actions::find_all(&mut conn, friend_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend_ideas))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);

    config.service(find_all_friend_ideas);
}
