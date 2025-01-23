use axum::Json;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{models::Station, schema};
pub fn get_stations(connection: &mut PgConnection) -> Json<Vec<Station>> {
    use schema::station::dsl::station;

    let stations = station
        .select(Station::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(stations)
}
