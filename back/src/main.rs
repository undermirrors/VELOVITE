mod api;
mod mock;
mod models;
mod populate;
mod schema;

use api::get_stations;
use axum::routing::get;
use axum::Router;

use crate::mock::get_stations_mock;
use crate::models::Station;
use crate::populate::populate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");

    let connection = &mut establish_connection();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error applying pending migrations");

    populate().await;

    use self::schema::station::dsl::*;
    let results = station
        .select(Station::as_select())
        .load(connection)
        .expect("Error loading posts");

    for s in results {
        println!("{:?}", s);
    }
    println!("Hello, world! bis");
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/stations", get(get_stations(connection)))
        .route("/mock/stations", get(get_stations_mock()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("API Server is listening on port 8000");
    axum::serve(listener, app).await.unwrap();
}

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
