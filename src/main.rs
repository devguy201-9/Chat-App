mod error;
mod router;
mod enums;
mod features;
mod utils;

use std::env;

use axum::Extension;
use dotenv::dotenv;
use error::{Error, Result};
use router::create_router;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    dotenv().ok();

    let opt = ConnectOptions::new(
        env::var("DATABASE_URL").map_err(|_| Error::EnvVarNotFound("DATABASE_URL".to_string()))?,
    );

    let db_connection = Database::connect(opt)
        .await
        .map_err(|_| Error::DatabaseConnectionFailed)?;

    // build our application with a route
    let app = create_router().layer(Extension(db_connection));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
