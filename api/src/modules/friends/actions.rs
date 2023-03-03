use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::{friends::models, schema::friends};

use super::models::{Friend, NewFriend, UpdateFriend};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_friends(conn: &mut PgConnection) -> Result<Vec<Friend>, DbError> {
    let all_friends = friends::table
        // .limit(10)
        .load::<models::Friend>(conn)
        .expect("Error loading friends");

    Ok(all_friends)
}

pub fn create_friend(conn: &mut PgConnection, new_friend: &NewFriend) -> Result<Friend, DbError> {
    let new_friend = Friend {
        id: Uuid::new_v4().to_string(),
        first_name: new_friend.first_name.clone(),
        last_name: new_friend.last_name.clone(),
        date_of_birth: new_friend.date_of_birth.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        met_at: new_friend.met_at.clone(),
        met_at_accuracy: new_friend.met_at_accuracy.clone(),
    };

    diesel::insert_into(friends::table)
        .values(&new_friend)
        .execute(conn)?;

    Ok(new_friend)
}

pub fn update_friend(
    conn: &mut PgConnection,
    id: String,
    update_friend: &UpdateFriend,
) -> Result<Friend, DbError> {
    let updated_friend = diesel::update(friends::table)
        .set((
            friends::first_name.eq(update_friend.first_name.clone()),
            friends::last_name.eq(update_friend.last_name.clone()),
            friends::date_of_birth.eq(update_friend.date_of_birth.clone()),
            friends::met_at.eq(update_friend.met_at.clone()),
            friends::met_at_accuracy.eq(update_friend.met_at_accuracy.clone()),
        ))
        .filter(friends::id.eq(id.clone()))
        .get_result(conn)?;

    Ok(updated_friend)
}

pub fn find_friend_by_id(conn: &mut PgConnection, id: String) -> Result<Friend, DbError> {
    let friend = friends::table
        .filter(friends::id.eq(id))
        .first(conn)
        .expect("Error loading friend");

    Ok(friend)
}
