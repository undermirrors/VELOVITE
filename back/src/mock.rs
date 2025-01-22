use crate::models::Station;
use axum::Json;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATIONS_MOCK: [Station; 2] = [
        Station {
            id: 0,
            name: "oui".to_string(),
            latitude: 26.0,
            longitude: 72.0,
            adress: "oui adresse".to_string(),
            area: "oui area".to_string(),
            capacity: 5,
        },
        Station {
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

pub fn get_stations_mock() -> Json<&'static [Station; 2]> {
    Json(&*STATIONS_MOCK)
}
