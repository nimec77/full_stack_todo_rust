use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TryIntoModel,
};

use crate::{
    database::{
        tasks::{self, Entity as Tasks, Model as TaskModel},
        users::Model as UserModel,
    },
    errors::app_error::AppError,
    routes::tasks::create_task_extractor::ValidateCreateTask,
};

pub async fn create_task(
    task_data: ValidateCreateTask,
    user: &UserModel,
    db: &DatabaseConnection,
) -> Result<TaskModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task_data.priority),
        title: Set(task_data.title.unwrap()),
        description: Set(task_data.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let new_task = new_task
        .save(db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    save_active_task(db, new_task).await
}

pub async fn find_task_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<TaskModel, AppError> {
    let task = Tasks::find_by_id(id)
        .filter(tasks::Column::UserId.eq(user_id))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?;

    task.ok_or_else(|| {
        eprintln!("Could not find task by id");
        AppError::new(StatusCode::NOT_FOUND, "Task not found")
    })
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<TaskModel, AppError> {
    let task = task
        .save(db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    convert_active_to_model(task)
}

pub async fn get_all_tasks(
    db: &DatabaseConnection,
    user_id: i32,
    get_deleted: bool,
) -> Result<Vec<TaskModel>, AppError> {
    let mut query = Tasks::find().filter(tasks::Column::UserId.eq(user_id));

    if !get_deleted {
        query = query.filter(tasks::Column::DeletedAt.is_null());
    }

    query.all(db).await.map_err(|error| {
        eprintln!("Error getting all tasks: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}

pub async fn get_default_tasks(db: &DatabaseConnection) -> Result<Vec<TaskModel>, AppError> {
    Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default tasks: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default tasks",
            )
        })
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TaskModel, AppError> {
    active_task.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
