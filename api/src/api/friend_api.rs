use crate::{models::friend_model::Friend, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::ObjectId;

use actix_web::{
    post, get, put,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/friends")]
pub async fn create_friend(db: Data<MongoRepo>, new_friend: Json<Friend>) -> HttpResponse {
    let data = Friend {
        id: None,
        first_name: new_friend.first_name.to_owned(),
        last_name: new_friend.last_name.to_owned(),
    };
    let friend_detail = db.create_friend(data).await;
    match friend_detail {
        Ok(friend) => HttpResponse::Ok().json(friend),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/friends/{id}")]
pub async fn update_friend(db: Data<MongoRepo>, path: Path<String>, updated_friend: Json<Friend>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = Friend {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        first_name: updated_friend.first_name.to_owned(),
        last_name: updated_friend.last_name.to_owned(),
    };
    let update_result = db.update_friend(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_friend_info = db.get_friend(&id).await;
                return match updated_friend_info {
                    Ok(friend) => HttpResponse::Ok().json(friend),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No friend found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/friends/{id}")]
pub async fn get_friend(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let friend_detail = db.get_friend(&id).await;
    match friend_detail {
        Ok(friend) => HttpResponse::Ok().json(friend),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/friends")]
pub async fn get_all_friends(db: Data<MongoRepo>) -> HttpResponse {
    let friends = db.get_all_friends().await;
    match friends {
        Ok(friends) => HttpResponse::Ok().json(friends),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
