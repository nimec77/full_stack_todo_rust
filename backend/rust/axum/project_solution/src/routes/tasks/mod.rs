pub mod create_task;
pub mod delete_task;
pub mod create_task_extractor;
pub mod get_all_tasks;
pub mod get_one_task;
pub mod update_tasks;

use crate::database::tasks::Model as TaskModel;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestTask {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTask {
    pub task: ResponseTask,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTaskList {
    pub tasks: Vec<ResponseTask>,
}
impl From<TaskModel> for ResponseTask {
    fn from(task: TaskModel) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            completed_at: task.completed_at.map(|time| time.to_string()),
        }
    }
}
