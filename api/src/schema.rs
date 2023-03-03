// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        date_of_birth -> Nullable<Date>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    posts,
    users,
);
