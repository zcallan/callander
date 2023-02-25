use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection, Database,
};

use crate::user::model::User;

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub async fn create(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
        };

        let user = self
            .db
            .collection("User")
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub async fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
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
            .db
            .collection("User")
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");

        Ok(updated_doc)
    }

    pub async fn find_one(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .db
            .collection("User")
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .db
            .collection("User")
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
}
