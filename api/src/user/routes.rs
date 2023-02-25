use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::bson::oid::ObjectId;

use actix_web::{
    post, get,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};

#[post("/users")]
pub async fn create(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
    };

    let user_detail = db.create_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/users/{id}")]
pub async fn update(db: Data<MongoRepo>, path: Path<String>, updated_user: Json<User>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        first_name: updated_user.first_name.to_owned(),
        last_name: updated_user.last_name.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn find_one(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let user_detail = db.get_user(&id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn find_all(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
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
