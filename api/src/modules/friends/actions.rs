use diesel::{pg::Pg, prelude::*, query_builder::Query};
use log::info;
use uuid::Uuid;

use crate::{friends::models, schema::friends};

use super::models::{Friend, NewFriend, UpdateFriend};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_by_id(conn: &mut PgConnection, id: String) -> Result<Friend, DbError> {
    let friend = friends::table
        .filter(friends::id.eq(id))
        .first(conn)
        .expect("Error loading friend");

    Ok(friend)
}

pub fn find_all(
    conn: &mut PgConnection,
    sort_by: String,
    sort_order: String,
) -> Result<Vec<Friend>, DbError> {
    let query = friends::table.limit(50);

    match sort_by.as_str() {
        "first_name" => {
            if sort_order == "desc" {
                query.order_by(friends::first_name.desc());
            } else {
                query.order_by(friends::first_name.asc());
            }
        }
        "last_name" => {
            if sort_order == "desc" {
                query.order_by(friends::last_name.desc());
            } else {
                query.order_by(friends::last_name.asc());
            }
        }
        "date_of_birth" => {
            if sort_order == "desc" {
                query.order_by(friends::date_of_birth.desc());
            } else {
                query.order_by(friends::date_of_birth.asc());
            }
        }
        "birthday" => {
            // query
            //     .order_by(friends::date_of_birth.asc())
            //     .where(diesel::dsl::date_part("month", friends::date_of_birth).eq(diesel::dsl::date_part("month", diesel::dsl::now())))
        }
        "met_at" => {
            if sort_order == "desc" {
                query.order_by(friends::met_at.desc());
            } else {
                query.order_by(friends::met_at.asc());
            }
        }
        _ => {
            if sort_order == "desc" {
                query.order_by(friends::created_at.desc());
            } else {
                query.order_by(friends::created_at.asc());
            }
        }
    };

    let all_friends = query
        .load::<models::Friend>(conn)
        .expect("Error loading friends");

    Ok(all_friends)
}

pub fn create(conn: &mut PgConnection, new_friend: &NewFriend) -> Result<Friend, DbError> {
    let friend = Friend {
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
        .values(&friend)
        .execute(conn)?;

    Ok(friend)
}

pub fn update(
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
