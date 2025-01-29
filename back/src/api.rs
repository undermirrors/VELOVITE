use std::collections::HashMap;

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
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Fetches the weather forecast data asynchronously.
///
/// # Returns
///
/// * `StatusCode::OK` with the forecast data in JSON format if successful.
/// * `StatusCode::INTERNAL_SERVER_ERROR` with an error message if the download fails.
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

/// Retrieves detailed information about all stations from the database.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection.
///
/// # Returns
///
/// * `StatusCode::OK` with the detailed station data in JSON format if successful.
/// * `StatusCode::INTERNAL_SERVER_ERROR` with an error message if the query fails.
pub async fn get_detailed_stations(State(state): State<AppState>) -> impl IntoResponse {
    let mut connection = state.connection.lock().unwrap();

    use schema::station::dsl::station;
    match station
        .select(DetailedStation::as_select())
        .load(&mut *connection)
    {
        Ok(stations) => (
            StatusCode::OK,
            Json(
                stations
                    .into_iter()
                    .map(|v| (v.id, v))
                    .collect::<HashMap<_, _>>(),
            ),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error querying database".to_owned(),
        )
            .into_response(),
    }
}

/// Retrieves basic information about all stations from the database.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection.
///
/// # Returns
///
/// * `StatusCode::OK` with the basic station data in JSON format if successful.
/// * `StatusCode::INTERNAL_SERVER_ERROR` with an error message if the query fails.
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

/// Retrieves detailed information about a specific station by its ID.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection.
/// * `id` - The ID of the station to retrieve.
///
/// # Returns
///
/// * `StatusCode::OK` with the detailed station data in JSON format if successful.
/// * `StatusCode::NOT_FOUND` with an error message if the station is not found.
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

/// Searches for stations by name using a case-insensitive partial match.
///
/// # Arguments
///
/// * `state` - The application state containing the database connection.
/// * `name` - The name or partial name of the station to search for.
///
/// # Returns
///
/// * `StatusCode::OK` with the matching station data in JSON format if successful.
/// * `StatusCode::NOT_FOUND` with an error message if no stations are found.
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

/// Parameters for the `predict` function, including station ID and date.
///
/// # Fields
///
/// * `id` - The ID of the station.
/// * `date` - The date and time for which to predict availability.
#[derive(Deserialize)]
pub struct PredictParams {
    id: u32,
    date: NaiveDateTime,
}

/// Parameters for the `predict` function, including station ID and date.
///
/// # Fields
///
/// * `date` - The date and time for which to predict availability.
#[derive(Deserialize)]
pub struct PredictionsParams {
    date: NaiveDateTime,
}

/// Data structure representing the availability of bikes and free stands at a station.
///
/// # Fields
///
/// * `id` - The ID of the station.
/// * `free_stands` - The number of free stands available.
/// * `available_bikes` - The number of bikes available.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailabilityData {
    id: u32,
    free_stands: u32,
    available_bikes: u32,
}

pub async fn predictions(
    State(data): State<AppState>,
    Query(params): Query<PredictionsParams>,
) -> impl IntoResponse {
    let is_holidays = data
        .holidays
        .par_iter()
        .any(|holiday| params.date.date() >= holiday.start && params.date.date() <= holiday.end);

    let forecast = download_weather_forecast().await.unwrap();
    let now = chrono::Utc::now().naive_utc();
    let weather_data = match forecast.get(&params.date.and_utc()) {
        Some(data) => data,
        None => {
            return (StatusCode::NOT_FOUND, "Weather data not found".to_owned()).into_response();
        }
    };
    let precipitation = weather_data.precipitation;
    let temperature = weather_data.temperature_2m;
    let wind_speed = weather_data.wind_speed_10m;

    let generated_data: Vec<Option<&MergedData>> = data
        .data
        .par_iter()
        .map(|(_, station_data)| {
            if params.date < now {
                station_data.par_iter().find_first(|d| {
                    d.month == params.date.month()
                        && d.day == params.date.day()
                        && d.hour == params.date.hour()
                        && d.week_day == params.date.weekday().num_days_from_monday()
                })
            } else {
                let wanted_point = MergedData {
                    id: station_data.first().unwrap().id,
                    hour: params.date.time().hour(),
                    day: params.date.day(),
                    month: params.date.month(),
                    week_day: params.date.weekday().num_days_from_monday(),
                    holidays: is_holidays,
                    free_stands: 0,
                    available_bikes: 0,
                    precipitation,
                    temperature,
                    wind_speed,
                };

                let nearest_data = station_data.par_iter().min_by(|a, b| {
                    distance(a, &wanted_point)
                        .partial_cmp(&distance(b, &wanted_point))
                        .unwrap()
                });

                nearest_data
            }
        })
        .collect();

    let mut response_data: HashMap<u32, AvailabilityData> = HashMap::new();
    generated_data.iter().for_each(|data| {
        if let Some(nearest) = data.as_ref() {
            response_data.insert(
                nearest.id,
                AvailabilityData {
                    id: nearest.id,
                    available_bikes: nearest.available_bikes,
                    free_stands: nearest.free_stands,
                },
            );
        }
    });

    if response_data.is_empty() {
        return (StatusCode::NOT_FOUND, "No data found".to_owned()).into_response();
    }

    (StatusCode::OK, Json(response_data)).into_response()
}

/// Predicts the availability of bikes and free stands at a station for a given date and time.
///
/// # Arguments
///
/// * `data` - The application state containing the prediction data and holidays.
/// * `params` - The parameters for the prediction, including station ID and date.
///
/// # Returns
///
/// * `StatusCode::OK` with the predicted availability data in JSON format if successful.
/// * `StatusCode::NOT_FOUND` with an error message if the station or weather data is not found.
pub async fn predict(
    State(data): State<AppState>,
    Query(params): Query<PredictParams>,
) -> impl IntoResponse {
    info!("🔍 Filter on the good station id");
    let station_data = match data.data.get(&params.id) {
        Some(data) => data,
        None => return (StatusCode::NOT_FOUND, "Station not found".to_owned()).into_response(),
    };

    let is_holidays = data
        .holidays
        .par_iter()
        .any(|holiday| params.date.date() >= holiday.start && params.date.date() <= holiday.end);

    let forecast = download_weather_forecast().await.unwrap();
    let now = chrono::Utc::now().naive_utc();
    if params.date < now {
        match station_data.par_iter().find_first(|d| {
            d.month == params.date.month()
                && d.day == params.date.day()
                && d.hour == params.date.hour()
                && d.week_day == params.date.weekday().num_days_from_monday()
        }) {
            Some(data) => {
                return (StatusCode::OK, Json(data)).into_response();
            }
            None => {
                return (StatusCode::NOT_FOUND, "Data not found".to_owned()).into_response();
            }
        }
    }

    let weather_data = match forecast.get(&params.date.and_utc()) {
        Some(data) => data,
        None => {
            return (StatusCode::NOT_FOUND, "Weather data not found".to_owned()).into_response();
        }
    };
    let precipitation = weather_data.precipitation;
    let temperature = weather_data.temperature_2m;
    let wind_speed = weather_data.wind_speed_10m;

    let mut wanted_point = MergedData {
        id: params.id,
        hour: params.date.time().hour(),
        day: params.date.day(),
        month: params.date.month(),
        week_day: params.date.weekday().num_days_from_monday(),
        holidays: is_holidays,
        free_stands: 0,
        available_bikes: 0,
        precipitation,
        temperature,
        wind_speed,
    };

    let nearest_data = station_data.par_iter().min_by(|a, b| {
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

    (
        StatusCode::OK,
        Json(AvailabilityData {
            id: wanted_point.id,
            available_bikes: wanted_point.available_bikes,
            free_stands: wanted_point.free_stands,
        }),
    )
        .into_response()
}
