use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{error, info};

const WEATHER_FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=45.7485&longitude=4.8467&hourly=temperature_2m,precipitation_probability,precipitation,weather_code,wind_speed_10m&timeformat=unixtime";
const WEATHER_URL: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast?latitude=45.7485&longitude=4.8467&start_date=2022-01-01&end_date=2025-01-23&hourly=temperature_2m,precipitation,wind_speed_10m&timeformat=unixtime&timezone=Europe%2FBerlin";
const VELOV_URL: &str = "https://data.grandlyon.com/fr/datapusher/ws/timeseries/jcd_jcdecaux.historiquevelov/all.json?filename=stations-velo-v-de-la-metropole-de-lyon---disponibilites-temps-reel";

/// Downloads weather forecast data from the specified URL and returns a `HashMap`
/// where the keys are `DateTime<Utc>` and the values are `WeatherData`.
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request fails.
/// - The response text cannot be read.
/// - The JSON response cannot be parsed.
///
/// # Returns
///
/// A `Result` containing a `HashMap` with `DateTime<Utc>` as keys and `WeatherData` as values,
/// or an error message if the operation fails.
///
/// # Example
///
/// ```rust
/// let weather_data = download_weather_forecast().await?;
/// ```
pub async fn download_weather_forecast() -> Result<HashMap<DateTime<Utc>, WeatherData>, &'static str>
{
    info!("🌤️🚀 Downloading weather forecast data...");
    let response = reqwest::get(WEATHER_FORECAST_URL)
        .await
        .map_err(|e| {
            error!("❌ Failed to download data: {}", e);
            "Failed to download data"
        })?
        .text()
        .await
        .map_err(|e| {
            error!("❌ Failed to read response text: {}", e);
            "Failed to read response text"
        })?;

    info!("📥 Downloaded weather forecast data...");
    let weather: WeatherRoot = serde_json::from_str(&response).map_err(|e| {
        error!("❌ Failed to parse JSON: {}", e);
        "Failed to parse JSON"
    })?;

    let weather_mapped: HashMap<DateTime<Utc>, WeatherData> = weather
        .hourly
        .time
        .iter()
        .zip(
            weather.hourly.temperature_2m.iter().zip(
                weather.hourly.precipitation.iter().zip(
                    weather.hourly.weather_code.unwrap().iter().zip(
                        weather
                            .hourly
                            .precipitation_probability
                            .unwrap()
                            .iter()
                            .zip(weather.hourly.wind_speed_10m.iter()),
                    ),
                ),
            ),
        )
        .map(
            |(
                time,
                (
                    temperature,
                    (precipitation, (weather_code, (precipitation_probability, wind_speed))),
                ),
            )| {
                (
                    *time,
                    WeatherData {
                        temperature_2m: *temperature,
                        precipitation_probability: Some(*precipitation_probability),
                        precipitation: *precipitation,
                        weather_code: Some(*weather_code),
                        wind_speed_10m: *wind_speed,
                    },
                )
            },
        )
        .collect();

    Ok(weather_mapped)
}

/// Downloads historical weather data from the specified URL and stores it in a JSON file.
///
/// # Errors
///
/// This function will log an error and return early if:
/// - The HTTP request fails.
/// - The response text cannot be read.
/// - The JSON response cannot be parsed.
/// - The data cannot be serialized to JSON.
/// - The JSON data cannot be written to a file.
///
/// # Example
///
/// ```rust
/// download_weather().await;
/// ```
pub async fn download_weather() {
    info!("🌤️🚀 Downloading weather data...");
    let response = match reqwest::get(WEATHER_URL).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => {
                error!("❌ Failed to read response text: {}", e);
                return;
            }
        },
        Err(e) => {
            error!("❌ Failed to download data: {}", e);
            return;
        }
    };

    info!("📥 Downloaded weather data...");
    let weather: WeatherRoot = match serde_json::from_str(&response) {
        Ok(weather) => weather,
        Err(e) => {
            error!("❌ Failed to parse JSON: {}", e);
            return;
        }
    };

    let weather_mapped: HashMap<DateTime<Utc>, WeatherData> = weather
        .hourly
        .time
        .iter()
        .zip(
            weather.hourly.temperature_2m.iter().zip(
                weather
                    .hourly
                    .precipitation
                    .iter()
                    .zip(weather.hourly.wind_speed_10m.iter()),
            ),
        )
        .map(|(time, (temperature, (precipitation, wind_speed)))| {
            (
                *time,
                WeatherData {
                    temperature_2m: *temperature,
                    precipitation_probability: None,
                    precipitation: *precipitation,
                    weather_code: None,
                    wind_speed_10m: *wind_speed,
                },
            )
        })
        .collect();

    // store the data in a json file
    let json = match serde_json::to_string(&weather_mapped) {
        Ok(json) => json,
        Err(e) => {
            error!("❌ Failed to serialize data to JSON: {}", e);
            return;
        }
    };
    if let Err(e) = std::fs::write("weather.json", json) {
        error!("❌ Failed to write data to file: {}", e);
    } else {
        info!("✅ Data successfully written to weather.json");
    }
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
pub async fn download_velov(max_velov_features: u32, velov_start: u32) {
    info!("🚴‍♂️🚀 Downloading velov data...");
    let mut index = velov_start;
    let mut url = format!(
        "{}?maxfeatures={}&start={}",
        VELOV_URL,
        max_velov_features,
        index * max_velov_features + 1
    );
    loop {
        info!("🚴‍♂️⏳ Downloading velov data... {}", index);
        let response = match reqwest::get(url).await {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("❌ Failed to read response text: {}", e);
                    break;
                }
            },
            Err(e) => {
                error!("❌ Failed to download data: {}", e);
                break;
            }
        };
        info!("📥 Downloaded velov data... {}", index);
        let raw_stations: VelovRoot = match serde_json::from_str(&response) {
            Ok(stations) => stations,
            Err(e) => {
                error!("❌ Failed to parse JSON: {}", e);
                break;
            }
        };

        // store the data in a json file in a separate thread
        // let data_clone = data.last().cloned();
        tokio::spawn(async move {
            let json = match serde_json::to_string(&raw_stations.values) {
                Ok(json) => json,
                Err(e) => {
                    error!("❌ Failed to serialize data to JSON: {}", e);
                    return;
                }
            };
            if let Err(e) = std::fs::write(
                format!(
                    "velov_datas/data-{}-{}.json",
                    (index - 1) * max_velov_features,
                    index * max_velov_features
                ),
                json,
            ) {
                error!("❌ Failed to write data to file: {}", e);
            } else {
                info!(
                    "✅ Data successfully written to data-{}-{}.json",
                    (index - 1) * max_velov_features,
                    index * max_velov_features
                );
            }
        });

        if raw_stations.next.is_none() {
            break;
        }

        url = raw_stations.next.unwrap();
        index += 1;
    }

    info!("📥 Downloaded velov data ✅");
}

/// Represents the root structure of weather data.
///
/// # Fields
///
/// * `latitude` - The latitude coordinate of the location.
/// * `longitude` - The longitude coordinate of the location.
/// * `generationtime_ms` - The time taken to generate the weather data, in milliseconds.
/// * `utc_offset_seconds` - The offset from UTC time, in seconds.
/// * `timezone` - The name of the timezone.
/// * `timezone_abbreviation` - The abbreviation of the timezone.
/// * `elevation` - The elevation of the location, in meters.
/// * `hourly_units` - The units used for the hourly weather data.
/// * `hourly` - The hourly weather data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherRoot {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    pub elevation: f64,
    #[serde(rename = "hourly_units")]
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

/// Represents hourly weather data including temperature, precipitation, weather code, and wind speed.
///
/// # Fields
///
/// * `time` - The time of the weather data.
/// * `temperature_2m` - The temperature at 2 meters above ground level.
/// * `precipitation_probability` - The probability of precipitation.
/// * `precipitation` - The amount of precipitation.
/// * `weather_code` - The weather code.
/// * `wind_speed_10m` - The wind speed at 10 meters above ground level.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourlyUnits {
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
    #[serde(rename = "precipitation_probability")]
    pub precipitation_probability: Option<String>,
    pub precipitation: String,
    pub weather_code: Option<String>,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: String,
}

/// Represents weather data including temperature, precipitation, weather code, and wind speed.
///
/// # Fields
///
/// * `temperature_2m` - The temperature at 2 meters above ground level.
/// * `precipitation` - The amount of precipitation.
/// * `weather_code` - The weather code.
/// * `wind_speed_10m` - The wind speed at 10 meters above ground level.
/// * `precipitation_probability` - The probability of precipitation.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature_2m: f32,
    pub precipitation: f32,
    pub weather_code: Option<i64>,
    pub wind_speed_10m: f32,
    pub precipitation_probability: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the root structure of weather data.
///
/// # Fields
///
/// * `latitude` - The latitude coordinate of the location.
/// * `longitude` - The longitude coordinate of the location.
/// * `generationtime_ms` - The time taken to generate the weather data, in milliseconds.
/// * `utc_offset_seconds` - The offset from UTC time, in seconds.
/// * `timezone` - The name of the timezone.
/// * `timezone_abbreviation` - The abbreviation of the timezone.
/// * `elevation` - The elevation of the location, in meters.
/// * `hourly_units` - The units used for the hourly weather data.
/// * `hourly` - The hourly weather data.
/// * `wind_speed_10m` - A vector of `f32` representing the wind speed at 10 meters above ground level.
pub struct Hourly {
    #[serde(with = "list_unix_time")]
    pub time: Vec<DateTime<Utc>>,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: Vec<f32>,
    #[serde(rename = "precipitation_probability")]
    pub precipitation_probability: Option<Vec<i64>>,
    pub precipitation: Vec<f32>,
    #[serde(rename = "weather_code")]
    pub weather_code: Option<Vec<i64>>,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: Vec<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the root structure of the Velov data.
///
/// This struct is used to deserialize JSON data from the Velov API.
///
/// # Fields
///
/// * `fields` - A vector of strings representing the fields in the data.
/// * `layer_name` - The name of the layer in the data. This field is renamed from `layer_name` in the JSON.
/// * `nb_results` - The number of results in the data. This field is renamed from `nb_results` in the JSON.
/// * `next` - An optional string representing the next set of data, if available.
/// * `table_alias` - An optional string representing the alias of the table. This field is renamed from `table_alias` in the JSON.
/// * `table_href` - A string representing the href of the table. This field is renamed from `table_href` in the JSON.
/// * `values` - A vector of `Value` representing the values in the data.
pub struct VelovRoot {
    pub fields: Vec<String>,
    #[serde(rename = "layer_name")]
    pub layer_name: String,
    #[serde(rename = "nb_results")]
    pub nb_results: u64,
    pub next: Option<String>,
    #[serde(rename = "table_alias")]
    pub table_alias: Option<String>,
    #[serde(rename = "table_href")]
    pub table_href: String,
    pub values: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a value with various attributes including date, stands information, and status.
///
/// # Fields
///
/// * `horodate` - The date and time associated with the value, formatted using `date_format`.
/// * `main_stands` - Information about the main stands, renamed to "main_stands" in serialized form.
/// * `number` - A unique identifier number for the value.
/// * `overflow_stands` - Optional information about overflow stands, renamed to "overflow_stands" in serialized form.
/// * `status` - The status of the value.
/// * `total_stands` - Information about the total stands, renamed to "total_stands" in serialized form.
pub struct Value {
    #[serde(with = "date_format")]
    pub horodate: DateTime<Utc>,
    #[serde(rename = "main_stands")]
    pub main_stands: MainStands,
    pub number: u16,
    #[serde(rename = "overflow_stands")]
    pub overflow_stands: Option<OverflowStands>,
    pub status: String,
    #[serde(rename = "total_stands")]
    pub total_stands: TotalStands,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the main stands of a station, including the availability of bikes and the total capacity.
///
/// # Fields
///
/// * `availabilities` - An instance of `Availabilities` that holds the current availability status of the stands.
/// * `capacity` - The total capacity of the stands, represented as a 16-bit unsigned integer.
pub struct MainStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the availability of different types of bikes and stands.
///
/// # Fields
///
/// * `bikes` - Total number of bikes available.
/// * `electrical_bikes` - Number of electrical bikes available.
/// * `electrical_internal_battery_bikes` - Number of electrical bikes with internal batteries available.
/// * `electrical_removable_battery_bikes` - Number of electrical bikes with removable batteries available.
/// * `mechanical_bikes` - Number of mechanical bikes available.
/// * `stands` - Number of stands available.
pub struct Availabilities {
    pub bikes: u16,
    pub electrical_bikes: u16,
    pub electrical_internal_battery_bikes: u16,
    pub electrical_removable_battery_bikes: u16,
    pub mechanical_bikes: u16,
    pub stands: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the overflow stands with their availabilities and capacity.
///
/// # Fields
///
/// * `availabilities` - The current availabilities of the overflow stands.
/// * `capacity` - The total capacity of the overflow stands.
pub struct OverflowStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the total stands information for a bike station.
///
/// # Fields
///
/// * `availabilities` - The availability status of the stands.
/// * `capacity` - The total capacity of the stands.
pub struct TotalStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

/// This module provides custom serialization and deserialization functions for `DateTime<Utc>`
/// using the `chrono` crate. The date format used is `%Y-%m-%d %H:%M:%S%:z`.
///
/// # Functions
///
/// - `serialize`: Serializes a `DateTime<Utc>` to a string using the specified format.
/// - `deserialize`: Deserializes a string to a `DateTime<Utc>` using the specified format.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::{Serialize, Deserialize};
/// use serde_json;
///
/// #[derive(Serialize, Deserialize)]
/// struct Example {
///     #[serde(with = "date_format")]
///     timestamp: DateTime<Utc>,
/// }
///
/// let example = Example {
///     timestamp: Utc::now(),
/// };
///
/// // Serialize to JSON
/// let json = serde_json::to_string(&example).unwrap();
///
/// // Deserialize from JSON
/// let deserialized: Example = serde_json::from_str(&json).unwrap();
/// ```
mod date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%:z";

    /// Serializes a `DateTime<Utc>` object into a string using the format "%Y-%m-%d %H:%M:%S%:z".
    /// The resulting string is then serialized using the provided serializer.
    ///
    /// # Arguments
    ///
    /// * `date` - The `DateTime<Utc>` object to serialize.
    ///
    /// * `serializer` - The serializer to use for serializing the string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized string or an error if serialization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::to_string;
    /// let date = Utc::now();
    /// let serialized_date = to_string(&date).unwrap();
    /// ```
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    /// Deserializes a string into a `DateTime<Utc>` object using the format "%Y-%m-%d %H:%M:%S%:z".
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The deserializer to use for deserializing the string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `DateTime<Utc>` object or an error if deserialization fails.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use serde_json::from_str;
    /// let date_str = "\"2023-10-05 12:34:56+00:00\"";
    /// let deserialized_date: DateTime<Utc> = from_str(date_str).unwrap();
    /// ```
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

/// This module provides custom serialization and deserialization functions for
/// a list of `DateTime<Utc>` objects to and from Unix timestamps.
///
/// # Functions
///
/// - `serialize`: Serializes a slice of `DateTime<Utc>` objects into a sequence of Unix timestamps.
/// - `deserialize`: Deserializes a sequence of Unix timestamps into a vector of `DateTime<Utc>` objects.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Example {
///     #[serde(with = "list_unix_time")]
///     dates: Vec<DateTime<Utc>>,
/// }
/// ```
mod list_unix_time {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, ser::SerializeSeq, Deserialize, Deserializer, Serializer};

    /// Serializes a slice of `DateTime<Utc>` objects into a sequence of Unix timestamps.
    /// The resulting sequence is then serialized using the provided serializer.
    /// The Unix timestamps are represented as integers.
    ///
    /// # Arguments
    ///
    /// * `date` - A slice of `DateTime<Utc>` objects to serialize.
    /// * `serializer` - The serializer to use for serializing the sequence.
    ///
    /// # Returns
    ///
    /// A `Result` containing the serialized sequence of Unix timestamps or an error if serialization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::to_string;
    /// let dates = vec![Utc::now(), Utc::now()];
    /// let serialized_dates = to_string(&dates).unwrap();
    /// ```
    pub fn serialize<S>(date: &[DateTime<Utc>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(date.len()))?;
        for d in date {
            seq.serialize_element(&d.timestamp())?;
        }
        seq.end()
    }

    /// Deserializes a sequence of Unix timestamps into a vector of `DateTime<Utc>` objects.
    /// The Unix timestamps are represented as integers.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The deserializer to use for deserializing the sequence.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized vector of `DateTime<Utc>` objects or an error if deserialization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use serde_json::from_str;
    /// let dates_str = "[1633420800, 1633424400]";
    /// let deserialized_dates: Vec<DateTime<Utc> = from_str(dates_str).unwrap();
    /// ```
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Vec::<i64>::deserialize(deserializer)?;
        let dt = s
            .iter()
            .map(|d| Utc.timestamp_opt(*d, 0).unwrap())
            .collect();
        Ok(dt)
    }
}
