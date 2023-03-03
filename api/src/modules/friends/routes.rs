use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    Error, HttpResponse,
};
use log::info;

use crate::db;
use crate::friends::actions;
use crate::friends::models::{Friend, NewFriend, UpdateFriend};

#[get("/friends")]
pub async fn find_all() -> Result<HttpResponse, Error> {
    let friends = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_all_friends(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friends))
}

#[get("/friends/{id}")]
pub async fn find_one(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend: Friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::find_friend_by_id(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[post("/friends")]
pub async fn create(new_friend: Json<NewFriend>) -> Result<HttpResponse, Error> {
    let friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::create_friend(&mut conn, &new_friend)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

#[post("/friends/{id}")]
pub async fn update(
    path: web::Path<String>,
    update_friend: Json<UpdateFriend>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let friend = web::block(move || {
        let mut conn = db::connection().expect("Error");
        actions::update_friend(&mut conn, id, &update_friend)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(friend))
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);
    // config.service(delete);
}
