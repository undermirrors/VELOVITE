use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::forecast)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a weather forecast.
/// 
/// # Fields
/// 
/// * `id` - The unique identifier for the forecast.
/// * `timestamp` - The date and time of the forecast.
/// * `available` - The number of available bikes.
pub struct Forecast {
    pub id: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub available: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::station)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a detailed station with various attributes.
///
/// # Fields
///
/// * `id` - The unique identifier for the station.
/// * `name` - The name of the station.
/// * `latitude` - The latitude coordinate of the station.
/// * `longitude` - The longitude coordinate of the station.
/// * `adress` - The address of the station.
/// * `area` - The area where the station is located.
/// * `capacity` - The capacity of the station, indicating how many bikes it can hold.
pub struct DetailedStation {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub adress: String,
    pub area: String,
    pub capacity: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::station)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a basic station with latitude and longitude coordinates.
/// 
/// # Fields
/// 
/// * `id` - The unique identifier for the station.
/// * `latitude` - The latitude coordinate of the station.
/// * `longitude` - The longitude coordinate of the station.
pub struct BasicStation {
    pub id: i32,
    pub latitude: f64,
    pub longitude: f64,
}
