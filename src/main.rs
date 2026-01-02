mod enums;
mod error;
mod features;
mod router;
mod socket;
mod utils;

use std::env;

use axum::{Extension, http::HeaderValue};
use dotenv::dotenv;
use error::{Error, Result};
use router::create_router;
use sea_orm::{ConnectOptions, Database};
use socketioxide::{SocketIo, handler::ConnectHandler};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::socket::{check_login, on_connect};

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

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect.with(check_login));

    // build our application with a route
    let app = create_router()
        .fallback_service(ServeDir::new("public"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                //.layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
                .layer(CorsLayer::new().allow_origin(HeaderValue::from_static("*")))
                .layer(Extension(db_connection))
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    Ok(())
}
