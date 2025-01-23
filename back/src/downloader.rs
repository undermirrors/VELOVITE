use ::serde::{Deserialize, Serialize};
use tracing::error;

pub async fn downloader_data() {
    println!("Downloading data...");
    let url = "https://data.grandlyon.com/fr/datapusher/ws/timeseries/jcd_jcdecaux.historiquevelov/all.json?maxfeatures=100&filename=stations-velo-v-de-la-metropole-de-lyon---disponibilites-temps-reel";
    let mut data: Vec<Value> = vec![];
    let mut index = 1;
    loop {
        let response = match reqwest::get(format!("{}&start={}", url, index)).await {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(e) => {
                    error!("Failed to read response text: {}", e);
                    break;
                }
            },
            Err(e) => {
                error!("Failed to download data: {}", e);
                break;
            }
        };
        println!("Downloaded data... {}", index);
        let mut raw_stations: Root = match serde_json::from_str(&response) {
            Ok(stations) => stations,
            Err(e) => {
                error!("Failed to parse JSON: {}", e);
                break;
            }
        };
        if raw_stations.next.is_empty() {
            break;
        }

        data.append(&mut raw_stations.values);

        index += 1;
    }

    // store the data in a json file
    let json = match serde_json::to_string(&data) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize data to JSON: {}", e);
            return;
        }
    };
    if let Err(e) = std::fs::write("data.json", json) {
        error!("Failed to write data to file: {}", e);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub fields: Vec<String>,
    #[serde(rename = "layer_name")]
    pub layer_name: String,
    #[serde(rename = "nb_results")]
    pub nb_results: i64,
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
    pub horodate: String,
    #[serde(rename = "main_stands")]
    pub main_stands: MainStands,
    pub number: i64,
    #[serde(rename = "overflow_stands")]
    pub overflow_stands: OverflowStands,
    pub status: String,
    #[serde(rename = "total_stands")]
    pub total_stands: TotalStands,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainStands {
    pub availabilities: Availabilities,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availabilities {
    pub bikes: i64,
    pub electrical_bikes: i64,
    pub electrical_internal_battery_bikes: i64,
    pub electrical_removable_battery_bikes: i64,
    pub mechanical_bikes: i64,
    pub stands: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverflowStands {
    pub availabilities: Availabilities,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalStands {
    pub availabilities: Availabilities,
    pub capacity: i64,
}
