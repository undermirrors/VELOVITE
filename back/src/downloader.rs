use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{error, info};

const WEATHER_FORECAST_URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=45.7485&longitude=4.8467&hourly=temperature_2m,precipitation_probability,precipitation,weather_code,wind_speed_10m&timeformat=unixtime";
const WEATHER_URL: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast?latitude=45.7485&longitude=4.8467&start_date=2022-01-01&end_date=2025-01-23&hourly=temperature_2m,precipitation,wind_speed_10m&timeformat=unixtime&timezone=Europe%2FBerlin";
const VELOV_URL: &str = "https://data.grandlyon.com/fr/datapusher/ws/timeseries/jcd_jcdecaux.historiquevelov/all.json?filename=stations-velo-v-de-la-metropole-de-lyon---disponibilites-temps-reel";

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherData {
    // #[serde(with = "ts_seconds")]
    // pub time: DateTime<Utc>,
    pub temperature_2m: f32,
    pub precipitation: f32,
    pub weather_code: Option<i64>,
    pub wind_speed_10m: f32,
    pub precipitation_probability: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct MainStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct OverflowStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalStands {
    pub availabilities: Availabilities,
    pub capacity: u16,
}

mod date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%:z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

mod list_unix_time {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, ser::SerializeSeq, Deserialize, Deserializer, Serializer};

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
