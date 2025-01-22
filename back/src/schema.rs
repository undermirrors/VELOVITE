// @generated automatically by Diesel CLI.

diesel::table! {
    forecasts (id, timestamp) {
        id -> Int4,
        timestamp -> Timestamp,
        available -> Int4,
    }
}

diesel::table! {
    stations (id) {
        id -> Int4,
        name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
        adress -> Varchar,
        area -> Varchar,
        capacity -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    forecasts,
    stations,
);
