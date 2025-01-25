use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::{error, info};

const WEATHER_URL: &str = "https://historical-forecast-api.open-meteo.com/v1/forecast?latitude=45.7485&longitude=4.8467&start_date=2022-01-01&end_date=2025-01-23&hourly=temperature_2m,precipitation,wind_speed_10m&timeformat=unixtime&timezone=Europe%2FBerlin";
const VELOV_DOWNLOAD_PER_PAGE: u64 = 500000;

pub async fn downloader_data() {
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

    // store the data in a json file
    let json = match serde_json::to_string(&weather) {
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

    info!("🚴‍♂️🚀 Downloading velov data...");
    let mut url = format!("https://data.grandlyon.com/fr/datapusher/ws/timeseries/jcd_jcdecaux.historiquevelov/all.json?maxfeatures={}&filename=stations-velo-v-de-la-metropole-de-lyon---disponibilites-temps-reel", VELOV_DOWNLOAD_PER_PAGE);
    let mut data: Vec<Value> = vec![];
    let mut index = 1;
    loop {
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
        let mut raw_stations: VelovRoot = match serde_json::from_str(&response) {
            Ok(stations) => stations,
            Err(e) => {
                error!("❌ Failed to parse JSON: {}", e);
                break;
            }
        };

        // store the data in a json file in a separate thread
        // let data_clone = data.last().cloned();
        let data_clone = raw_stations.values.clone();
        let index_clone = index;
        tokio::spawn(async move {
            let json = match serde_json::to_string(&data_clone) {
                Ok(json) => json,
                Err(e) => {
                    error!("❌ Failed to serialize data to JSON: {}", e);
                    return;
                }
            };
            if let Err(e) = std::fs::write(
                format!(
                    "datas/data-{}-{}.json",
                    (index_clone - 1) * VELOV_DOWNLOAD_PER_PAGE,
                    index_clone * VELOV_DOWNLOAD_PER_PAGE
                ),
                json,
            ) {
                error!("❌ Failed to write data to file: {}", e);
            } else {
                info!(
                    "✅ Data successfully written to data-{}-{}.json",
                    (index_clone - 1) * VELOV_DOWNLOAD_PER_PAGE,
                    index_clone * VELOV_DOWNLOAD_PER_PAGE
                );
            }
        });

        if raw_stations.next.is_empty() {
            break;
        }
        url = raw_stations.next;

        data.append(&mut raw_stations.values);

        index += 1;
    }

    // store the data in a json file
    let json = match serde_json::to_string(&data) {
        Ok(json) => json,
        Err(e) => {
            error!("❌ Failed to serialize data to JSON: {}", e);
            return;
        }
    };
    if let Err(e) = std::fs::write("data.json", json) {
        error!("❌ Failed to write data to file: {}", e);
    } else {
        info!("✅ Data successfully written to data.json");
    }
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
    pub precipitation: String,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hourly {
    #[serde(with = "list_unix_time")]
    pub time: Vec<DateTime<Utc>>,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: Vec<f64>,
    pub precipitation: Vec<f64>,
    #[serde(rename = "wind_speed_10m")]
    pub wind_speed_10m: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VelovRoot {
    pub fields: Vec<String>,
    #[serde(rename = "layer_name")]
    pub layer_name: String,
    #[serde(rename = "nb_results")]
    pub nb_results: u64,
    pub next: String,
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
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &[DateTime<Utc>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.iter().map(|d| d.timestamp()).collect::<Vec<i64>>();
        let json = serde_json::to_string(&s).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&json)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Vec::deserialize(deserializer)?;
        let dt = s
            .iter()
            .map(|d| Utc.timestamp_opt(*d, 0).unwrap())
            .collect();
        Ok(dt)
    }
}
