use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::forecast)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Forecast {
    pub id: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub available: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::station)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
pub struct BasicStation {
    pub id: i32,
    pub latitude: f64,
    pub longitude: f64,
}