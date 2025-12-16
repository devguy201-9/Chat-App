use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Environment variable errors
    #[error("Environment variable {0} not found")]
    EnvVarNotFound(String),

    // Database errors
    #[error("Database connection failed")]
    DatabaseConnectionFailed,
    #[error("Insert failed: {0}")]
    InsertFailed(#[source] sea_orm::error::DbErr),
    #[error("Query failed {0}")]
    QueryFailed(#[source] sea_orm::error::DbErr),
    #[error("Update failed: {0}")]
    UpdateFailed(#[source] sea_orm::error::DbErr),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Delete failed: {0}")]
    DeleteFailed(#[source] sea_orm::error::DbErr),

    // File errors
    #[error("Create file failed")]
    CreateFileFailed,

    #[error("File type invalid")]
    FileTypeInvalid,

    // JWT errors
    #[error("JWT decode failed: {0}")]
    DecodeJwtFailed(String),

    // Auth errors
    #[error("Please login first")]
    TokenNotFound,

    #[error("{0}")]
    Unknown(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResp {
            status: String,
            message: String,
        }

        let status = match &self {
            Error::TokenNotFound | Error::DecodeJwtFailed(_) => StatusCode::UNAUTHORIZED,
            Error::RecordNotFound => StatusCode::NOT_FOUND,
            Error::InsertFailed(_)
            | Error::QueryFailed(_)
            | Error::UpdateFailed(_)
            | Error::DeleteFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseConnectionFailed => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResp {
            status: status.to_string(),
            message: self.to_string(),
        });

        (status, body).into_response()

        /*(
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResp {
                status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                message: self.to_string(),
            }),
        )
            .into_response()*/
    }
}
