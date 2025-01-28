use crate::models::{BasicStation, DetailedStation};
use axum::Json;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATIONS_MOCK: [BasicStation; 2] = [
        BasicStation {
            id: 7055,
            latitude: 45.74676623,
            longitude: 4.8420507,
        },
        BasicStation {
            id: 8052,
            latitude: 45.74708261,
            longitude: 4.86019085,
        }
    ];
}

lazy_static! {
    static ref DETAILED_STATIONS_MOCK: [DetailedStation; 2] = [
        DetailedStation {
            id: 7055,
            name: "oui".to_string(),
            latitude: 45.74676623,
            longitude: 4.8420507,
            adress: "oui adresse".to_string(),
            area: "oui area".to_string(),
            capacity: 16,
        },
        DetailedStation {
            id: 8052,
            name: "non".to_string(),
            latitude: 45.74708261,
            longitude: 4.86019085,
            adress: "non adresse".to_string(),
            area: "non area".to_string(),
            capacity: 16,
        }
    ];
}

pub fn get_detailed_stations_mock() -> Json<&'static [DetailedStation; 2]> {
    Json(&*DETAILED_STATIONS_MOCK)
}

pub fn get_stations_mock() -> Json<&'static [BasicStation; 2]> {
    Json(&*STATIONS_MOCK)
}
