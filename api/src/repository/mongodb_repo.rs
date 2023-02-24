use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::user_model::User;
use crate::models::friend_model::Friend;

pub struct MongoRepo {
    users: Collection<User>,
    friends: Collection<Friend>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("callander");
        let users: Collection<User> = db.collection("User");
        let friends: Collection<Friend> = db.collection("Friend");
        MongoRepo { users, friends }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
        };
        let user = self
            .users
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "first_name": new_user.first_name,
                    "last_name": new_user.last_name,
                },
        };
        let updated_doc = self
            .users
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .users
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .users
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn create_friend(&self, new_friend: Friend) -> Result<InsertOneResult, Error> {
        let new_doc = Friend {
            id: None,
            first_name: new_friend.first_name,
            last_name: new_friend.last_name,
        };
        let friend = self
            .friends
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating friend");
        Ok(friend)
    }

    pub async fn update_friend(&self, id: &String, new_friend: Friend) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_friend.id,
                    "first_name": new_friend.first_name,
                    "last_name": new_friend.last_name,
                },
        };
        let updated_doc = self
            .friends
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating friend");
        Ok(updated_doc)
    }

    pub async fn get_friend(&self, id: &String) -> Result<Friend, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let friend_detail = self
            .friends
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting friend's detail");

        Ok(friend_detail.unwrap())
    }

    pub async fn get_all_friends(&self) -> Result<Vec<Friend>, Error> {
        let mut cursors = self
            .friends
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of friends");
        let mut friends: Vec<Friend> = Vec::new();
        while let Some(friend) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            friends.push(friend)
        }
        Ok(friends)
    }
}
