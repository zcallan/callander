use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection, Database,
};

use crate::friend::model::Friend;

pub async fn create(db: Database, new_friend: Friend) -> Result<InsertOneResult, Error> {
    let new_doc = Friend {
        id: None,
        first_name: new_friend.first_name,
        last_name: new_friend.last_name,
    };
    let friend = db
        .collection("Friend")
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creating friend");
    Ok(friend)
}

pub async fn update(db: Database, id: &String, new_friend: Friend) -> Result<UpdateResult, Error> {
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

    let updated_doc = db
        .collection("Friend")
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Error updating friend");
    Ok(updated_doc)
}

pub async fn find_one(db: Database, id: &String) -> Result<Friend, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let friend_detail = db
        .collection("Friend")
        .find_one(filter, None)
        .await
        .ok()
        .expect("Error getting friend's detail");

    Ok(friend_detail.unwrap())
}

pub async fn find_all(db: Database) -> Result<Vec<Friend>, Error> {
    use futures::TryStreamExt;

    let mut cursors: mongodb::Cursor<Friend> = db
        .collection("Friend")
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
