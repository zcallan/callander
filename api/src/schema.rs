// @generated automatically by Diesel CLI.

diesel::table! {
    friends (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        date_of_birth -> Nullable<Date>,
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
    users,
);
