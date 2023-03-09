use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::friends;

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize, Deserialize, Clone, TS)]
#[ExistingTypePath = "crate::schema::sql_types::MetAtAccuracyEnum"]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum MetAtAccuracyEnum {
    Day,
    Month,
    Year,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS, Clone)]
#[diesel(table_name = friends)]
#[ts(export)]
pub struct Friend {
    pub id: String,
    pub first_name: String,
    pub last_name: String,

    #[ts(type = "string")]
    pub date_of_birth: Option<NaiveDate>,

    #[ts(type = "string")]
    pub created_at: NaiveDateTime,

    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,

    #[ts(type = "string")]
    pub met_at: Option<NaiveDate>,
    pub met_at_accuracy: Option<MetAtAccuracyEnum>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewFriend {
    pub first_name: String,
    pub last_name: String,

    #[ts(type = "string")]
    pub date_of_birth: Option<NaiveDate>,

    #[ts(type = "string")]
    pub met_at: Option<NaiveDate>,
    pub met_at_accuracy: Option<MetAtAccuracyEnum>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateFriend {
    pub first_name: String,
    pub last_name: String,

    #[ts(type = "string")]
    pub date_of_birth: Option<NaiveDate>,

    #[ts(type = "string")]
    pub met_at: Option<NaiveDate>,
    pub met_at_accuracy: Option<MetAtAccuracyEnum>,
}

pub struct FriendsFindAllSort {
    pub sort_by: String,
    pub sort_dir: String,
}
