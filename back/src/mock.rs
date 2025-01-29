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

/// Downloads Velov bike-sharing data from the specified URL in a paginated manner and stores each page in a separate JSON file.
///
/// # Arguments
///
/// * `max_velov_features` - The maximum number of features to download per request.
/// * `velov_start` - The starting index for pagination.
///
/// # Errors
///
/// This function will log an error and break the loop if:
/// - The HTTP request fails.
/// - The response text cannot be read.
/// - The JSON response cannot be parsed.
/// - The data cannot be serialized to JSON.
/// - The JSON data cannot be written to a file.
///
/// # Example
///
/// ```rust
/// download_velov(100, 0).await;
/// ```
/// Returns a JSON response containing a mock array of detailed stations.
///
/// # Example
///
/// ```
/// use axum::Json;
/// use crate::mock::get_detailed_stations_mock;
///
/// let detailed_stations = get_detailed_stations_mock();
/// assert_eq!(detailed_stations.0.len(), 2);
/// ```
///
/// # Returns
///
/// A `Json` response containing a reference to a static array of `DetailedStation` objects.
pub fn get_detailed_stations_mock() -> Json<&'static [DetailedStation; 2]> {
    Json(&*DETAILED_STATIONS_MOCK)
}

/// Returns a JSON response containing a mock array of basic stations.
///
/// # Example
///
/// ```
/// use axum::Json;
/// use crate::mock::get_stations_mock;
///
/// let stations = get_stations_mock();
/// assert_eq!(stations.0.len(), 2);
/// ```
///
/// # Returns
///
/// A `Json` response containing a reference to a static array of `BasicStation` objects.
pub fn get_stations_mock() -> Json<&'static [BasicStation; 2]> {
    Json(&*STATIONS_MOCK)
}
