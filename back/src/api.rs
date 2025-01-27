use crate::downloader::download_weather_forecast;
use crate::models::BasicStation;
use crate::{models::DetailedStation, schema};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use diesel::{
    ExpressionMethods, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use std::sync::{Arc, Mutex};

pub async fn get_weather_forecast() -> impl IntoResponse {
    let forecast_data = download_weather_forecast().await;
    match forecast_data {
        Ok(forecast) => (StatusCode::OK, Json(forecast)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error downloading weather forecast".to_owned(),
        )
            .into_response(),
    }
}
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
        .first(&mut *connection)
    {
        Ok(s) => (StatusCode::OK, Json(s)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found".to_owned()).into_response(),
    }
}

pub async fn search_station(
    State(connection): State<Arc<Mutex<PgConnection>>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let mut connection = connection.lock().unwrap();

    use schema::station::dsl::station;
    match station
        .select(DetailedStation::as_select())
        .filter(schema::station::name.ilike("%".to_owned() + &name + "%"))
        .limit(5)
        .load(&mut *connection)
    {
        Ok(s) => (StatusCode::OK, Json(s)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Not found".to_owned()).into_response(),
    }
}
