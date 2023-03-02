use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::friends;
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS)]
#[ts(export)]
pub struct Friend {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    #[ts(type = "string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewFriend {
    pub first_name: String,
    pub last_name: String,
    #[ts(type = "string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
}
