mod mock;
mod models;
mod populate;
mod schema;

use axum::routing::get;
use axum::{Json, Router};

use crate::mock::get_stations_mock;
use crate::models::Station;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");

    let connection = &mut PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error applying pending migrations");

    use self::schema::stations::dsl::*;
    let results = stations
        .limit(5)
        .select(Station::as_select())
        .load(connection)
        .expect("Error loading posts");

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/stations", get(get_stations_mock()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("API Server is listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
