// @generated automatically by Diesel CLI.

diesel::table! {
    calendars (id) {
        id -> Int4,
        name -> Varchar,
        uuid -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
