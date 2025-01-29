mod api;
mod args;
mod downloader;
mod learning;
mod mock;
mod models;
mod populate;
mod schema;
mod utils;

use api::{get_detailed_stations, get_weather_forecast, predict, predictions};
use args::Args;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use downloader::{download_velov, download_weather};
use indoc::indoc;
use learning::{
    benchmark, filter_velov_data, merge_data, read_merged_data_from_file, MergedData,
    SchoolHolidays,
};
use mock::get_detailed_station_mock;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::api::{get_detailed_station, get_stations, search_station};
use crate::mock::{get_detailed_stations_mock, get_stations_mock};
use crate::populate::populate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/// Application state.
///
/// # Fields
///
/// * `connection` - The database connection.
/// * `data` - The merged data.
/// * `holidays` - The school holidays.
#[derive(Clone)]
pub struct AppState {
    connection: Arc<Mutex<PgConnection>>,
    data: Arc<HashMap<u32, Vec<MergedData>>>,
    holidays: Arc<Vec<SchoolHolidays>>,
}

/// Main function.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    if args.benchmark {
        benchmark();
        return;
    }
    if args.filter_velov_data {
        filter_velov_data();
        return;
    }
    if args.merge_datas {
        merge_data();
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

    if args.populate {
        populate().await;
    }

    if args.mock {
        info!("🚀 Running in mock mode!");
    } else {
        info!("🌐 Running in normal mode!");
    }

    let app_state = AppState {
        connection: Arc::new(Mutex::new(establish_connection())),
        data: Arc::new(read_merged_data_from_file(if !args.mock {
            "merged_data"
        } else {
            "merged_data_mock"
        })),
        holidays: Arc::new(
            serde_json::from_str(&std::fs::read_to_string("school_holidays.json").unwrap())
                .unwrap(),
        ),
    };

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                indoc! {"🚴‍♂️ Welcome to VELOVITE! 🌟
                        You are on the API side of the project. 🌐
                        You can, for example, hit: 
                        🌦️ /weather_forecast 
                        🚉 /stations 
                        🔍 /search/:name 
                        📊 /station/:id 
                        🔮 and the most important: /predict 🔮
                        Enjoy exploring our API! 🎉
                        🚴‍♂️🚴‍♀️🚴‍♂️🚴‍♀️🚴‍♂️🚴‍♀️🚴‍♂️🚴‍♀️🚴‍♂️🚴‍♀️
                        🌟🌟🌟🌟🌟🌟🌟🌟🌟🌟
                        🌐🌐🌐🌐🌐🌐🌐🌐🌐🌐
                        🌦️🌦️🌦️🌦️🌦️🌦️🌦️🌦️🌦️🌦️
                        🚉🚉🚉🚉🚉🚉🚉🚉🚉🚉
                        🔍🔍🔍🔍🔍🔍🔍🔍🔍🔍
                        📊📊📊📊📊📊📊📊📊📊
                        🔮🔮🔮🔮🔮🔮🔮🔮🔮🔮
                        🎉🎉🎉🎉🎉🎉🎉🎉🎉🎉
                        "}
            }),
        )
        .route("/weather_forecast", get(get_weather_forecast))
        .route("/search/:name", get(search_station))
        .route("/predict", get(predict))
        .route("/predictions", get(predictions))
        .with_state(app_state.clone())
        .layer(CorsLayer::permissive());

    let app = if args.mock {
        app.merge(mock_router())
    } else {
        app.merge(normal_router(app_state))
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("API Server is listening on port 8000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Establishes a connection to the database.
///
/// # Returns
///
/// * A connection to the database.
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

/// Function to listen for shutdown signals.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C/SIGINT");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl-C received, shutting down");
        }
        _ = terminate => {
            tracing::info!("SIGTERM received, shutting down");
        }
    }
}

/// Mock router.
///
/// # Returns
///
/// * A router with mock data.
fn mock_router() -> Router {
    Router::new()
        .route("/detailed_stations", get(get_detailed_stations_mock()))
        .route("/stations", get(get_stations_mock()))
        .route("/station/:id", get(get_detailed_station_mock))
        .layer(CorsLayer::permissive())
}

/// Normal router with database connection.
///
/// # Arguments
///
/// * `app_state` - The application state containing the database connection.
///
/// # Returns
///
/// * A router with the database connection.
fn normal_router(app_state: AppState) -> Router {
    Router::new()
        .route("/detailed_stations", get(get_detailed_stations))
        .route("/stations", get(get_stations))
        .route("/station/:id", get(get_detailed_station))
        .with_state(app_state)
        .layer(CorsLayer::permissive())
}
