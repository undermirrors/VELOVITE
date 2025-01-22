use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::forecasts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Forecast {
    pub id: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub available: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::stations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Station {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub adress: String,
    pub area: String,
    pub capacity: i32,
}