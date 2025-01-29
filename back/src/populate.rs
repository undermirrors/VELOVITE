use crate::models::DetailedStation;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

const URL: &str = "https://data.grandlyon.com/geoserver/metropole-de-lyon/ows?SERVICE=WFS&VERSION=2.0.0&request=GetFeature&typename=metropole-de-lyon:pvo_patrimoine_voirie.pvostationvelov&outputFormat=application/json&SRSNAME=EPSG:4171&sortBy=gid";

/// Populates the database with Velov station data.
pub async fn populate() {
    info!("🚴 Populating the database 🌐");

    let response = reqwest::get(URL).await.unwrap().text().await.unwrap();
    let raw_stations: StationsData = serde_json::from_str(&response).unwrap();

    // convert the stations raw data into Stations struct
    let stations: Vec<DetailedStation> = raw_stations
        .features
        .iter()
        .map(|station| DetailedStation {
            id: station.properties.idstation,
            name: station.properties.nom.clone(),
            latitude: station.geometry.coordinates[1],
            longitude: station.geometry.coordinates[0],
            adress: station.properties.adresse1.clone(),
            area: station.properties.commune.clone(),
            capacity: station.properties.nbbornettes,
        })
        .collect();

    // insert the stations into the database
    let database_url = std::env::var("DATABASE_URL").expect("Database URL must be set");
    let connection = &mut diesel::pg::PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // delete all stations first
    diesel::delete(crate::schema::station::table)
        .execute(connection)
        .expect("Error deleting stations");

    // insert the new stations
    diesel::insert_into(crate::schema::station::table)
        .values(&stations)
        .execute(connection)
        .expect("Error inserting stations");
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the Velov station data.
///
/// # Fields
///
/// * `type` - The type of the data.
/// * `features` - The features of the data.
/// * `total_features` - The total number of features.
/// * `number_matched` - The number of matched features.
/// * `number_returned` - The number of returned features.
/// * `time_stamp` - The timestamp of the data.
/// * `crs` - The coordinate reference system.
/// * `bbox` - The bounding box of the data.
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

/// Represents a feature of the Velov station data.
///
/// # Fields
///
/// * `type` - The type of the feature.
/// * `id` - The ID of the feature.
/// * `geometry` - The geometry of the feature.
/// * `geometry_name` - The name of the geometry.
/// * `properties` - The properties of the feature.
/// * `bbox` - The bounding box of the feature.
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
/// Represents the geometry of a feature.
///
/// # Fields
///
/// * `type` - The type of the geometry.
/// * `coordinates` - The coordinates of the geometry.
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the properties of a feature.
///
/// # Fields
///
/// * `idstation` - The ID of the station.
/// * `nom` - The name of the station.
/// * `adresse1` - The first address of the station.
/// * `adresse2` - The second address of the station.
/// * `commune` - The commune where the station is located.
/// * `numdansarrondissement` - The number of the station in the arrondissement.
/// * `nbbornettes` - The number of bornettes at the station.
/// * `stationbonus` - The station bonus.
/// * `pole` - The pole of the station.
/// * `ouverte` - Whether the station is open.
/// * `achevement` - The achievement of the station.
/// * `validite` - The validity of the station.
/// * `gid` - The GID of the station.
/// * `code_insee` - The INSEE code of the station.
pub struct Properties {
    pub idstation: i32,
    pub nom: String,
    pub adresse1: String,
    pub adresse2: Option<String>,
    pub commune: String,
    pub numdansarrondissement: Option<i64>,
    pub nbbornettes: i32,
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
/// Represents the coordinate reference system.
///
/// # Fields
///
/// * `type` - The type of the coordinate reference system.
/// * `properties` - The properties of the coordinate reference system.
pub struct Crs {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a propertie.
///
/// # Fields
///
/// * `name` - The name.
pub struct Properties2 {
    pub name: String,
}
