use crate::database::users;
use crate::database::users::ActiveModel as UserActiveModel;
use crate::database::users::Entity as Users;
use crate::database::users::Model as UserModel;
use crate::errors::app_error::AppError;
use axum::http::StatusCode;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::TryIntoModel;

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: UserActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })?;

    convert_active_to_model(user)
}

pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<UserModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by username: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?
        .ok_or_else(|| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Incorrect username and/or password",
            )
        })
}

fn convert_active_to_model(active_user: UserActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}

pub(crate) async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
    Users::delete_by_id(id).exec(db).await.map_err(|error| {
        eprintln!("Error deleting user: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(())
}
