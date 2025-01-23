use crate::models::BasicStation;
use crate::{models::DetailedStation, schema};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use std::sync::{Arc, Mutex};

pub async fn get_detailed_stations(
    State(connection): State<Arc<Mutex<PgConnection>>>,
) -> Json<Vec<DetailedStation>> {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    let stations = station
        .select(DetailedStation::as_select())
        .load(&mut *connection)
        .expect("Error loading posts");

    Json(stations)
}

pub async fn get_stations(
    State(connection): State<Arc<Mutex<PgConnection>>>,
) -> Json<Vec<BasicStation>> {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    let stations = station
        .select(BasicStation::as_select())
        .load(&mut *connection)
        .expect("Error loading posts");

    Json(stations)
}

pub async fn get_detailed_station(
    State(connection): State<Arc<Mutex<PgConnection>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    match station
        .select(DetailedStation::as_select())
        .filter(schema::station::id.eq(id))
        .limit(1)
        .load(&mut *connection)
        .expect(&("Error loading station with id ".to_owned() + &id.to_string()))
        .first()
    {
        Some(s) => (StatusCode::OK, Json(s)).into_response(),
        None => (StatusCode::NOT_FOUND, "Not found".to_string()).into_response(),
    }
}
