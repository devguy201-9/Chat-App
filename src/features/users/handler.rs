use axum::{
    Extension, Json, extract::Multipart, extract::Path, http::StatusCode, response::IntoResponse,
};

use bcrypt::{DEFAULT_COST, hash};
use std::path::Path as StdPath;
use regex::Regex;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use super::model::{CreateUser, UpdateUser, UserDTO};
use crate::error::{Error, Result};

use entity::user;

pub async fn create_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse> {
    // hash password before saving
    let password_hash =
        hash(&payload.password, DEFAULT_COST).map_err(|e| Error::Unknown(e.to_string()))?;

    let user_model = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(password_hash),
        is_online: Set(payload.is_online),
        ..Default::default()
    };

    user_model
        .insert(&db_connection)
        .await
        .map_err(|e| Error::InsertFailed(e))?;

    Ok((
        StatusCode::CREATED,
        Json(json!(
            {
                "message": "User created successfully"
            }
        )),
    ))
}

pub async fn get_user_by_id(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    let result = UserDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        avatar: user.avatar,
        is_online: user.is_online,
    };

    Ok((StatusCode::OK, Json(result)))
}

pub async fn update_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse> {
    let mut user: user::ActiveModel = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?
        .into();

    user.name = Set(payload.name.unwrap());
    user.email = Set(payload.email.unwrap());
    user.avatar = Set(payload.avatar);

    user.update(&db_connection)
        .await
        .map_err(|e| Error::UpdateFailed(e))?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User updated successfully"
            }
        )),
    ))
}

pub async fn delete_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let mut user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(id)))
        .one(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .ok_or_else(|| Error::RecordNotFound)?;

    user::Entity::delete_by_id(user.id)
        .exec(&db_connection)
        .await
        .map_err(|e| Error::DeleteFailed(e))?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!(
            {
                "message": "User deleted successfully"
            }
        )),
    ))
}

pub async fn get_all_users(
    Extension(db_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse> {
    let users: Vec<UserDTO> = user::Entity::find()
        .all(&db_connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?
        .into_iter()
        .map(|user| UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            is_online: user.is_online,
        })
        .collect();

    Ok((StatusCode::ACCEPTED, Json(users)))
}

pub async fn update_avatar(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?
    {
        let field_name = field.name().map(|s| s.to_string()).unwrap_or_default();

        if field_name == "avatar" {
            let file_name = field
                .file_name()
                .ok_or_else(|| Error::Unknown("Missing file name".to_string()))?;
            let content_type = field
                .content_type()
                .ok_or_else(|| Error::Unknown("Missing content type".to_string()))?
                .to_string();
            
            let regex = Regex::new(mime::IMAGE_STAR.as_ref())
               .map_err(|e| Error::Unknown(e.to_string()))?;

            if regex.is_match(&content_type) {
                let mut user: user::ActiveModel = user::Entity::find()
                    .filter(Condition::all().add(user::Column::Id.eq(id)))
                    .one(&db_connection)
                    .await
                    .map_err(|e| Error::QueryFailed(e))?
                    .ok_or_else(|| Error::RecordNotFound)?
                    .into();

                // sanitize file name and generate unique name
                let sanitized = StdPath::new(file_name)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| Error::Unknown("Invalid file name".to_string()))?;
                let unique_name = format!("{}_{}", Uuid::new_v4(), sanitized);

                user.avatar = Set(Some(unique_name.clone()));

                // create directories if needed
                tokio::fs::create_dir_all("./public/uploads")
                    .await
                    .map_err(|_| Error::CreateFileFailed)?;

                let mut file = File::create(format!("./public/uploads/{file_name}"))
                    .await
                    .map_err(|_| Error::CreateFileFailed)?;
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| Error::Unknown(e.to_string()))?;

                file.write(&data)
                    .await
                    .map_err(|e| Error::Unknown(e.to_string()))?;

                user.update(&db_connection)
                    .await
                    .map_err(|e| Error::UpdateFailed(e))?;
            } else {
                return Err(Error::FileTypeInvalid);
            }
        }
    }

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Avatar updated successfully" })),
    ))
}
