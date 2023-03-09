use diesel::prelude::*;
use log::info;
use uuid::Uuid;

use crate::schema::friends_ideas;

use super::models::{FriendsIdea, NewFriendsIdea, UpdateFriendsIdea};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_by_id(conn: &mut PgConnection, idea_id: String) -> Result<FriendsIdea, DbError> {
    let friend = friends_ideas::table
        .filter(friends_ideas::id.eq(idea_id))
        .first(conn)
        .expect("Error loading friend idea");

    Ok(friend)
}

pub fn find_all(conn: &mut PgConnection, friend_id: String) -> Result<Vec<FriendsIdea>, DbError> {
    let all_friend_ideas = friends_ideas::table
        // .limit(10)
        .filter(friends_ideas::friend_id.eq(friend_id))
        .load::<FriendsIdea>(conn)
        .expect("Error loading friend ideas");

    Ok(all_friend_ideas)
}

pub fn create(
    conn: &mut PgConnection,
    new_friend_idea: &NewFriendsIdea,
) -> Result<FriendsIdea, DbError> {
    let friends_idea = FriendsIdea {
        id: Uuid::new_v4().to_string(),
        content: new_friend_idea.content.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        friend_id: new_friend_idea.friend_id.clone(),
        idea_type: new_friend_idea.idea_type.clone(),
    };

    diesel::insert_into(friends_ideas::table)
        .values(&friends_idea)
        .execute(conn)?;

    Ok(friends_idea)
}

pub fn update(
    conn: &mut PgConnection,
    idea_id: String,
    update_friend_idea: &UpdateFriendsIdea,
) -> Result<FriendsIdea, DbError> {
    let updated_friend_idea = diesel::update(friends_ideas::table)
        .set((
            friends_ideas::content.eq(update_friend_idea.content.clone()),
            friends_ideas::idea_type.eq(update_friend_idea.idea_type.clone()),
        ))
        .filter(friends_ideas::friend_id.eq(idea_id.clone()))
        .get_result(conn)?;

    Ok(updated_friend_idea)
}
