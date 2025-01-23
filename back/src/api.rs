use axum::Json;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{models::DetailedStation, schema};
use crate::models::BasicStation;

pub fn get_detailed_stations(connection: &mut PgConnection) -> Json<Vec<DetailedStation>> {
    use schema::station::dsl::station;

    let stations = station
        .select(DetailedStation::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(stations)
}

pub fn get_stations(connection: &mut PgConnection) -> Json<Vec<BasicStation>> {
    use schema::station::dsl::station;

    let stations = station
        .select(BasicStation::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(stations)
}
