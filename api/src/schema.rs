// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "friends_events_type_enum"))]
    pub struct FriendsEventsTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "friends_ideas_type_enum"))]
    pub struct FriendsIdeasTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "met_at_accuracy_enum"))]
    pub struct MetAtAccuracyEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MetAtAccuracyEnum;

    friends (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        date_of_birth -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        met_at -> Nullable<Date>,
        met_at_accuracy -> Nullable<MetAtAccuracyEnum>,
        user_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FriendsEventsTypeEnum;

    friends_events (id) {
        id -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        friend_id -> Varchar,
        event_type -> FriendsEventsTypeEnum,
        user_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FriendsIdeasTypeEnum;

    friends_ideas (id) {
        id -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        friend_id -> Varchar,
        idea_type -> FriendsIdeasTypeEnum,
        user_id -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Varchar,
        title -> Varchar,
        body -> Text,
        for_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Varchar,
    }
}

diesel::joinable!(friends_events -> friends (friend_id));
diesel::joinable!(friends_ideas -> friends (friend_id));

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    friends_events,
    friends_ideas,
    posts,
);
