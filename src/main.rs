pub mod enums;
pub mod features;

mod router;
mod error;
use std::env;

use axum::Extension;
use dotenv::dotenv;
use router::create_router;
use sea_orm::{ConnectOptions, Database};

#[tokio::main]
async fn main() {
    // ...

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    dotenv().ok();

    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").unwrap());

    let db_connection = match Database::connect(opt).await {
        Ok(conn) => conn,
        Err(e) => panic!("{}", format!("Database connection failed: {:?}", e)),
    };

    // build our application with a route
    let app = create_router().layer(Extension(db_connection));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
