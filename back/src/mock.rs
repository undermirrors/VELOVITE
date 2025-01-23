use crate::models::{BasicStation, DetailedStation};
use axum::Json;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATIONS_MOCK: [BasicStation; 2] = [
        BasicStation {
            id: 0,
            latitude: 26.0,
            longitude: 72.0,
        },
        BasicStation {
            id: 1,
            latitude: 4.0,
            longitude: 12.0,
        }
    ];
}


lazy_static! {
    static ref DETAILED_STATIONS_MOCK: [DetailedStation; 2] = [
        DetailedStation {
            id: 0,
            name: "oui".to_string(),
            latitude: 26.0,
            longitude: 72.0,
            adress: "oui adresse".to_string(),
            area: "oui area".to_string(),
            capacity: 5,
        },
        DetailedStation {
            id: 1,
            name: "non".to_string(),
            latitude: 4.0,
            longitude: 12.0,
            adress: "non adresse".to_string(),
            area: "non area".to_string(),
            capacity: 10,
        }
    ];
}


pub fn get_detailed_stations_mock() -> Json<&'static [DetailedStation; 2]> {
    Json(&*DETAILED_STATIONS_MOCK)
}

pub fn get_stations_mock() -> Json<&'static [BasicStation; 2]> {
    Json(&*STATIONS_MOCK)
}
