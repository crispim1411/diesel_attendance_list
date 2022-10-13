// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        creator -> Varchar,
        server_id -> Varchar,
        date_created -> Date,
        expiration -> Int4,
    }
}

diesel::table! {
    events_test (id) {
        id -> Int4,
        name -> Varchar,
        creator -> Varchar,
        server_id -> Varchar,
        date_created -> Date,
        expiration -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        mention -> Varchar,
        event_id -> Int4,
    }
}

diesel::table! {
    users_test (id) {
        id -> Int4,
        name -> Varchar,
        mention -> Varchar,
        event_id -> Int4,
    }
}

diesel::joinable!(users -> events (event_id));
diesel::joinable!(users_test -> events_test (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    events_test,
    users,
    users_test,
);
