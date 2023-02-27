use chrono::{NaiveDate, NaiveDateTime};
use diesel::{data_types::PgDate, sql_types::Date};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS)]
#[ts(export)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    #[ts(type = "string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    #[ts(type = "string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
}
