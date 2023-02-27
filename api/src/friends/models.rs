use chrono::NaiveDate;
use diesel::data_types::PgDate;
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

// impl Friend {
//     pub fn find_one(id: Uuid) -> Result<Vec<Self>, CustomError> {
//         use crate::friends::schema::dsl::*;

//         let conn = db::connection()?;
//         let friends = friends::table.load::<Friend>(&conn)?;
//         Ok(friends)
//     }

//     pub fn find_all() -> Result<Vec<Self>, CustomError> {
//         use crate::schema::friends::dsl::*;

//         let conn = db::connection()?;
//         let friends = friends::table.load::<Friend>(&conn)?;
//         Ok(friends)
//     }
// }
