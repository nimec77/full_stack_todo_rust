use crate::{
    database::{
        tasks::{self},
        users::{ActiveModel as UserModel, Model},
    },
    errors::app_error::AppError,
    queries::{task_queries, user_queries},
    utilities::{hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use super::{RequestUser, ResponseDataUser, ResponseUser};

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
    };
    let user = user_queries::save_active_user(&db, new_user).await?;

    create_default_tasks(&user, &db).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser::from(user),
    }))
}

async fn create_default_tasks(user: &Model, db: &DatabaseConnection) -> Result<(), AppError> {
    let default_tasks = task_queries::get_default_tasks(db).await?;

    for default_task in default_tasks {
        let new_task = tasks::ActiveModel {
            user_id: Set(Some(user.id)),
            title: Set(default_task.title),
            description: Set(default_task.description),
            completed_at: Set(default_task.completed_at),
            deleted_at: Set(default_task.deleted_at),
            ..Default::default()
        };
        new_task.save(db).await.map_err(|error| {
            eprintln!("Error creating default task {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
    }

    Ok(())
}
