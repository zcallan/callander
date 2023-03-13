use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::friends_ideas;

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone, TS)]
#[ExistingTypePath = "crate::schema::sql_types::FriendsIdeasTypeEnum"]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum FriendIdeaTypeEnum {
    Gift,
    Conversation,
    Activity,
    Place,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS)]
#[ts(export)]
pub struct FriendsIdea {
    pub id: String,
    pub content: String,

    #[ts(type = "string")]
    pub created_at: NaiveDateTime,

    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,

    pub friend_id: String,
    pub idea_type: FriendIdeaTypeEnum,

    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewFriendsIdea {
    pub friend_id: String,
    pub content: String,
    pub idea_type: FriendIdeaTypeEnum,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateFriendsIdea {
    pub content: String,
    pub idea_type: FriendIdeaTypeEnum,
}
