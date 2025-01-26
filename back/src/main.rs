mod api;
mod args;
mod downloader;
mod learning;
mod mock;
mod models;
mod populate;
mod schema;

use api::get_detailed_stations;
use args::Args;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use downloader::{download_velov, download_weather};
use learning::filter_velov_data;
use tower_http::cors::CorsLayer;

use crate::api::{get_detailed_station, get_stations, search_station};
use crate::mock::{get_detailed_stations_mock, get_stations_mock};
use crate::populate::populate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use std::env;
use std::sync::{Arc, Mutex};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    if args.filter_velov_data {
        info!("🧹 Filtering velov data...");
        filter_velov_data();
        return;
    }

    if args.download_weather_data {
        download_weather().await;
        return;
    }
    if args.download_velov_data {
        download_velov(args.max_velov_features, args.velov_start).await;
        return;
    }

    let connection = Arc::new(Mutex::new(establish_connection()));

    if args.populate {
        populate().await;
    }

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/stations", get(get_stations))
        .route("/mock/stations", get(get_stations_mock()))
        .route("/detailed_stations", get(get_detailed_stations))
        .route("/mock/detailed_stations", get(get_detailed_stations_mock()))
        .route("/station/:id", get(get_detailed_station))
        .route("/search/:name", get(search_station))
        .with_state(connection)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("API Server is listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error applying pending migrations");

    connection
}
