use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::posts;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, TS)]
#[ts(export)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    #[ts(type = "string")]
    pub for_date: NaiveDate,
    #[ts(type = "string")]
    pub created_at: NaiveDateTime,
    #[ts(type = "string")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    #[ts(type = "string")]
    pub for_date: NaiveDate,
}
