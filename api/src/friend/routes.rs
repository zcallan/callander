use mongodb::{bson::oid::ObjectId, Client};

use actix_web::{
    get, post,
    web::{self, Data, Json, Path, ServiceConfig},
    HttpResponse,
};

#[post("/friends")]
pub async fn create(db: web::Data<Client>, new_friend: Json<Friend>) -> HttpResponse {
    let data = Friend {
        id: None,
        first_name: new_friend.first_name.to_owned(),
        last_name: new_friend.last_name.to_owned(),
    };

    let friend_detail = friend::repository::create(db, data).await;

    match friend_detail {
        Ok(friend) => HttpResponse::Ok().json(friend),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/friends/{id}")]
pub async fn update(
    db: Data<MongoRepo>,
    path: Path<String>,
    updated_friend: Json<Friend>,
) -> HttpResponse {
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
pub async fn find_one(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
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
pub async fn find_all(db: Data<MongoRepo>) -> HttpResponse {
    let friends = db.get_all_friends().await;
    match friends {
        Ok(friends) => HttpResponse::Ok().json(friends),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);
    // config.service(delete);
}
