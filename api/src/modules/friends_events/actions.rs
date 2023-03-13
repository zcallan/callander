use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::schema::friends_events;

use super::models::{FriendsEvent, NewFriendsEvent, UpdateFriendsEvent};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_by_id(
    conn: &mut PgConnection,
    user_id: String,
    event_id: String,
) -> Result<FriendsEvent, DbError> {
    let friend = friends_events::table
        .filter(friends_events::id.eq(event_id))
        .filter(friends_events::user_id.eq(user_id))
        .first(conn)
        .expect("Error loading friend event");

    Ok(friend)
}

pub fn find_all(
    conn: &mut PgConnection,
    user_id: String,
    friend_id: String,
) -> Result<Vec<FriendsEvent>, DbError> {
    let all_friend_events = friends_events::table
        // .limit(10)
        .filter(friends_events::friend_id.eq(friend_id))
        .filter(friends_events::user_id.eq(user_id))
        .load::<FriendsEvent>(conn)
        .expect("Error loading friend events");

    Ok(all_friend_events)
}

pub fn create(
    conn: &mut PgConnection,
    user_id: String,
    new_friend_event: &NewFriendsEvent,
) -> Result<FriendsEvent, DbError> {
    let friends_event = FriendsEvent {
        id: Uuid::new_v4().to_string(),
        content: new_friend_event.content.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        friend_id: new_friend_event.friend_id.clone(),
        event_type: new_friend_event.event_type.clone(),
        user_id: user_id.clone(),
    };

    diesel::insert_into(friends_events::table)
        .values(&friends_event)
        .execute(conn)?;

    Ok(friends_event)
}

pub fn update(
    conn: &mut PgConnection,
    user_id: String,
    event_id: String,
    update_friend_event: &UpdateFriendsEvent,
) -> Result<FriendsEvent, DbError> {
    let updated_friend_event = diesel::update(friends_events::table)
        .set((
            friends_events::content.eq(update_friend_event.content.clone()),
            friends_events::event_type.eq(update_friend_event.event_type.clone()),
        ))
        .filter(friends_events::id.eq(event_id))
        .filter(friends_events::user_id.eq(user_id))
        .get_result(conn)?;

    Ok(updated_friend_event)
}
