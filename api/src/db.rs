use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::user::repository::UserRepository;

pub struct Db {
    users: UserRepository,
    friends: UserRepository,
}

impl Db {
    pub async fn init(&self) -> Self {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");

        let db = client.database("callander");

        Db {
            users: UserRepository { db: &db },
            friends: UserRepository { db: &db },
        }
    }
}
