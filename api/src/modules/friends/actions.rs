use diesel::prelude::*;
use uuid::Uuid;

use crate::{friends::models, schema::friends};

use super::models::{Friend, NewFriend};

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
    };

    diesel::insert_into(friends::table)
        .values(&new_friend)
        .execute(conn)?;

    Ok(new_friend)
}

pub fn find_friend_by_id(conn: &mut PgConnection, id: String) -> Result<Friend, DbError> {
    let friend = friends::table
        .filter(friends::id.eq(id))
        .first(conn)
        .expect("Error loading friend");

    Ok(friend)
}
