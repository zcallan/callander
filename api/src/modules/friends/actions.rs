use diesel::expression_methods::ExpressionMethods;
use diesel::prelude::PgConnection;
use uuid::Uuid;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::{friends::models, schema::friends};

use super::models::FriendsFindAllSort;
use super::models::{Friend, NewFriend, UpdateFriend};
use super::utils::sort_by_column;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_by_id(conn: &mut PgConnection, user_id: String, id: String) -> Result<Friend, DbError> {
    let friend = friends::table
        .filter(friends::user_id.eq(user_id))
        .filter(friends::id.eq(id))
        .first(conn)?;

    Ok(friend)
}

pub fn find_all(
    conn: &mut PgConnection,
    user_id: String,
    sort: FriendsFindAllSort,
) -> Result<Vec<Friend>, DbError> {
    let mut query = friends::table
        .filter(friends::user_id.eq(user_id))
        .into_boxed();

    query = match sort.sort_by.as_ref() {
        "first_name" => sort_by_column(query, friends::first_name, sort.sort_dir),
        "last_name" => sort_by_column(query, friends::last_name, sort.sort_dir),
        "date_of_birth" => sort_by_column(query, friends::date_of_birth, sort.sort_dir),
        "met_at" => sort_by_column(query, friends::met_at, sort.sort_dir),
        _ => sort_by_column(query, friends::created_at, sort.sort_dir),
    };

    let all_friends: Vec<Friend> = query.load::<models::Friend>(conn)?;

    Ok(all_friends)
}

pub fn create(
    conn: &mut PgConnection,
    user_id: String,
    new_friend: &NewFriend,
) -> Result<Friend, DbError> {
    let friend = Friend {
        id: Uuid::new_v4().to_string(),
        first_name: new_friend.first_name.clone(),
        last_name: new_friend.last_name.clone(),
        date_of_birth: new_friend.date_of_birth.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        met_at: new_friend.met_at.clone(),
        met_at_accuracy: new_friend.met_at_accuracy.clone(),
        user_id: user_id.clone(),
    };

    diesel::insert_into(friends::table)
        .values(&friend)
        .execute(conn)?;

    Ok(friend)
}

pub fn update(
    conn: &mut PgConnection,
    user_id: String,
    id: String,
    update_friend: &UpdateFriend,
) -> Result<Friend, DbError> {
    let updated_friend = diesel::update(friends::table)
        .filter(friends::id.eq(id))
        .filter(friends::user_id.eq(user_id))
        .set((
            friends::first_name.eq(update_friend.first_name.clone()),
            friends::last_name.eq(update_friend.last_name.clone()),
            friends::date_of_birth.eq(update_friend.date_of_birth.clone()),
            friends::met_at.eq(update_friend.met_at.clone()),
            friends::met_at_accuracy.eq(update_friend.met_at_accuracy.clone()),
        ))
        .get_result(conn)?;

    Ok(updated_friend)
}
