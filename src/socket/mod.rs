use axum::http::header::AUTHORIZATION;
use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait};
use socketioxide::extract::SocketRef;
use tracing::info;

use crate::error::{Error, Result};
use crate::socket::handler::{handle_join, handle_message, handler_disconnect};
use crate::utils::jwt::decode_jwt;

pub mod handler;
pub mod model;

pub async fn on_connect(socket: SocketRef) {
    info!("socket connected {}", socket.id);

    socket.on("message", handle_message);
    socket.on("join", handle_join);
    socket.on_disconnect(handler_disconnect);
}

// middleware
pub async fn check_login(socket: SocketRef) -> Result<()> {
    let token = socket
        .req_parts()
        .headers
        .get(AUTHORIZATION)
        .ok_or_else(|| Error::TokenNotFound)?
        .to_str()
        .or_else(|e| Err(Error::Unknown(e.to_string())))?;

    let token = token.replace("Bearer ", "");

    // decode jwt -> user_id
    let user_id = decode_jwt(token)?;

    // get DB connection from socket extensions
    let db_connection = socket
        .req_parts()
        .extensions
        .get::<DatabaseConnection>()
        .ok_or_else(|| Error::DatabaseConnectionFailed)?;

    // verify user exists
    let _user = user::Entity::find_by_id(user_id)
        .one(db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    Ok(())
}
