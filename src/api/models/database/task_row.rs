use juniper::GraphQLObject;

use crate::api::models::Task;
use crate::api::schema::*;

#[derive(GraphQLObject, Queryable, Insertable, Clone)]
#[table_name = "tasks"]
pub struct TaskRow {
    pub uuid: String,
    pub content: String,
    pub priority: i32,
    pub tags: Option<String>,
    pub is_complete: bool,
    pub parent_list_uuid: String,
    pub creation_information_uuid: String
}

impl TaskRow {
    pub fn from_task(task: Task) -> Result<TaskRow, String> {
        // Convert tags to json
        let json_tags: Option<String>;
        match task.tags {
            Some(tags) => {
                match serde_json::to_string(&tags) {
                    Ok(res) => {
                        json_tags = Some(res);
                    },
                    Err(_) => {
                        return Err(String::from("Error serializing tags to json"));
                    }
                }
            },
            None => {
                json_tags = None;
            }
        }
        // Convert priority to primitive
        let priority = num_traits::ToPrimitive::to_i32(&task.priority);
        Ok(TaskRow {
            uuid: task.uuid.to_string(),
            content: task.content,
            priority: priority.unwrap_or(1), // Default to normal
            tags: json_tags,
            is_complete: task.is_complete,
            parent_list_uuid: task.parent_list_uuid.to_string(),
            creation_information_uuid: task.creation_information_uuid.to_string()
        })
    }
}