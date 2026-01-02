use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use bcrypt::verify;
use entity::user;

use crate::{
    error::{Error, Result},
    utils::jwt,
};

use super::model::{LoginRequest, LoginResponse};

pub async fn login(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let LoginRequest { email, password } = payload;
    // find user by email only, then verify password
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        //.filter(user::Column::Password.eq(password))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or(Error::RecordNotFound)?;

    let valid = verify(&password, &user.password).map_err(|e| Error::Unknown(e.to_string()))?;

    if !valid {
        return Err(Error::RecordNotFound); // invalid credentials
    }

    let token = jwt::encode_jwt(user.id)?;

    let resp = LoginResponse {
        msg: String::from("Login Successfully!"),
        token: token,
    };

    Ok((StatusCode::CREATED, Json(resp)))
}
