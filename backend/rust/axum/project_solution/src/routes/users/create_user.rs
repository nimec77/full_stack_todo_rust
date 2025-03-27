use crate::{
    database::{tasks::{self, Entity as Tasks}, users::{ActiveModel as UserModel, Model}},
    errors::app_error::AppError,
    utilities::{hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};

use super::{convert_active_to_model, RequestUser, ResponseDataUser, ResponseUser};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(token_wrapper): State<TokenWrapper>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let hashed_password = hash_password(&user.password)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let token = create_token(&token_wrapper, &user.username)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let new_user = UserModel {
        username: Set(user.username),
        password: Set(hashed_password),
        token: Set(Some(token)),
        deleted_at: Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|error: DbErr| {
        println!("Error creating user: {:?}", &error);
        let error_message = error.to_string();
        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(StatusCode::BAD_REQUEST, "Username already taken, try again with a different user name")
        } else {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    })?;

    let user = convert_active_to_model(new_user)?;

    create_default_tasks(&user, &db).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser::from(user),
    }))
}

async fn create_default_tasks(user: &Model, db: &DatabaseConnection) -> Result<(), AppError> {
    let default_tasks = Tasks::find()
    .filter(tasks::Column::IsDefault.eq(Some(true)))
    .all(db)
    .await
    .map_err(|error| {
        eprintln!("Error getting default tasks {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    for task in default_tasks {
        let new_task = tasks::ActiveModel {
            user_id: Set(Some(user.id)),
            title: Set(task.title),
            description: Set(task.description),
            completed_at: Set(task.completed_at),
            deleted_at: Set(task.deleted_at),
            ..Default::default()
        };
        new_task.save(db).await.map_err(|error| {
            eprintln!("Error creating default task {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
    }

    Ok(())
}
