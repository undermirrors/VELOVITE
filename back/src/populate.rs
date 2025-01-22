use reqwest::Url;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

const URL: &str = "https://data.grandlyon.com/geoserver/metropole-de-lyon/ows?SERVICE=WFS&VERSION=2.0.0&request=GetFeature&typename=metropole-de-lyon:pvo_patrimoine_voirie.pvostationvelov&outputFormat=application/json&SRSNAME=EPSG:4171&sortBy=gid";

pub async fn populate(){
    let response = reqwest::get(URL).await.unwrap().text().await.unwrap();
    let stations: StationsData = serde_json::from_str(&response).unwrap();
    println!("{:?}", stations);
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationsData {
    #[serde(rename = "type")]
    pub type_field: String,
    pub features: Vec<Feature>,
    pub total_features: i64,
    pub number_matched: i64,
    pub number_returned: i64,
    pub time_stamp: String,
    pub crs: Crs,
    pub bbox: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub geometry: Geometry,
    #[serde(rename = "geometry_name")]
    pub geometry_name: String,
    pub properties: Properties,
    pub bbox: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub idstation: i64,
    pub nom: String,
    pub adresse1: String,
    pub adresse2: Option<String>,
    pub commune: String,
    pub numdansarrondissement: Option<i64>,
    pub nbbornettes: i64,
    pub stationbonus: Value,
    pub pole: Option<String>,
    pub ouverte: bool,
    pub achevement: String,
    pub validite: String,
    pub gid: i64,
    #[serde(rename = "code_insee")]
    pub code_insee: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crs {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties2 {
    pub name: String,
}
