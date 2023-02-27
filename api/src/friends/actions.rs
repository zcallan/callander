use diesel::prelude::*;
use uuid::Uuid;

use crate::friends::models;

use super::models::{Friend, NewFriend};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_friends(conn: &mut PgConnection) -> Result<Vec<Friend>, DbError> {
    use crate::schema::friends::dsl::*;

    let all_friends = friends
        // .limit(10)
        .load::<models::Friend>(conn)
        // .execute(conn)
        .expect("Error loading posts");

    // .filter(id.eq(uid.to_string()))
    // .first::<models::Friend>(conn)
    // .optional()?;

    Ok(all_friends)
}

pub fn create_friend(conn: &mut PgConnection, new_friend: &NewFriend) -> Result<Friend, DbError> {
    use crate::schema::friends::dsl::*;

    let new_friend = Friend {
        id: Uuid::new_v4().to_string(),
        first_name: new_friend.first_name.clone(),
        last_name: new_friend.last_name.clone(),
        date_of_birth: new_friend.date_of_birth.clone(),
    };

    diesel::insert_into(friends)
        .values(&new_friend)
        .execute(conn)?;

    // diesel::insert_into(friends)
    //     .values(&new_friend)
    //     .get_result(conn)
    //     .expect("Error saving new friend");

    // .filter(id.eq(uid.to_string()))
    // .first::<models::Friend>(conn)
    // .optional()?;

    Ok(new_friend)
}

// pub fn find_friend_by_uid(
//     conn: &mut SqliteConnection,
//     uid: Uuid,
// ) -> Result<Option<models::Friend>, DbError> {
//     use crate::schema::friends::dsl::*;

//     let friend = friends
//         .filter(id.eq(uid.to_string()))
//         .first::<models::Friend>(conn)
//         .optional()?;

//     Ok(friend)
// }
