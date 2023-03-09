use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::friends_events;

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone, TS)]
#[ExistingTypePath = "crate::schema::sql_types::FriendsEventsTypeEnum"]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum FriendEventTypeEnum {
    Gift,
    Conversation,
    Activity,
    Place,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS)]
#[ts(export)]
pub struct FriendsEvent {
    pub id: String,
    pub content: String,

    #[ts(type = "string")]
    pub created_at: NaiveDateTime,

    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,

    pub friend_id: String,
    pub event_type: FriendEventTypeEnum,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewFriendsEvent {
    pub friend_id: String,
    pub content: String,
    pub event_type: FriendEventTypeEnum,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateFriendsEvent {
    pub content: String,
    pub event_type: FriendEventTypeEnum,
}
