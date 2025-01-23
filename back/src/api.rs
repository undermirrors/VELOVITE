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
) -> impl IntoResponse {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    match station
        .select(DetailedStation::as_select())
        .load(&mut *connection)
    {
        Ok(stations) => (StatusCode::OK, Json(stations)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error querying database".to_owned(),
        )
            .into_response(),
    }
}

pub async fn get_stations(State(connection): State<Arc<Mutex<PgConnection>>>) -> impl IntoResponse {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    match station
        .select(BasicStation::as_select())
        .load(&mut *connection)
    {
        Ok(stations) => (StatusCode::OK, Json(stations)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error querying db".to_owned(),
        )
            .into_response(),
    }
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
        None => (StatusCode::NOT_FOUND, "Not found".to_owned()).into_response(),
    }
}
