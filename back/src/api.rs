use crate::downloader::download_weather_forecast;
use crate::learning::MergedData;
use crate::models::BasicStation;
use crate::utils::distance;
use crate::AppState;
use crate::{models::DetailedStation, schema};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{Datelike, NaiveDateTime, Timelike};
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::info;

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
pub async fn get_detailed_stations(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = state.connection.lock().unwrap();

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

pub async fn get_stations(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = state.connection.lock().unwrap();

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
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut connection = state.connection.lock().unwrap();

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
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let mut connection = state.connection.lock().unwrap();

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

pub async fn predict(
    State(data): State<AppState>,
    Query(id): Query<u32>,
    Query(date): Query<NaiveDateTime>,
) -> impl IntoResponse {
    info!("🔍 Filter on the good station id");
    let station_data = match data.data.get(&id) {
        Some(data) => data,
        None => return (StatusCode::NOT_FOUND, "Station not found".to_owned()).into_response(),
    };

    let is_holidays = data
        .holidays
        .iter()
        .any(|holiday| date.date() >= holiday.start && date.date() <= holiday.end);

    //todo!("use the weather prediction");
    let precipitation = 12.0;
    let temperature = 20.0;
    let wind_speed = 10.0;

    let mut wanted_point = MergedData {
        id,
        hour: date.time().hour(),
        day: date.day(),
        month: date.month(),
        week_day: date.weekday().num_days_from_monday(),
        holidays: is_holidays,
        free_stands: 0,
        available_bikes: 0,
        precipitation,
        temperature,
        wind_speed,
    };

    let nearest_data = station_data.iter().min_by(|a, b| {
        distance(a, &wanted_point)
            .partial_cmp(&distance(b, &wanted_point))
            .unwrap()
    });

    if let Some(nearest) = nearest_data {
        wanted_point.free_stands = nearest.free_stands;
        wanted_point.available_bikes = nearest.available_bikes;
        info!("🎯 Nearest point found: {:?}", nearest);
    } else {
        return (StatusCode::NOT_FOUND, "No data found".to_owned()).into_response();
    }

    (StatusCode::OK, Json(wanted_point)).into_response()
}
