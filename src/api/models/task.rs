use uuid::Uuid;

use crate::api::models::database::TaskRow;

#[derive(FromPrimitive, ToPrimitive)]
pub enum TaskPriority {
    LOW = 0,
    NORMAL = 1,
    HIGH = 2,
}

pub struct Task {
    pub uuid: Uuid,
    pub content: String,
    pub priority: TaskPriority,
    pub tags: Option<Vec<String>>,
    pub is_complete: bool,
    pub parent_list_uuid: Uuid,
    pub creation_information_uuid: Uuid
}

impl Task {
    pub fn from_task_row(task_row: TaskRow) -> Result<Task, String> {
        // Parse uuid
        let uuid: Uuid;
        match Uuid::parse_str(&task_row.uuid) {
            Ok(res) => {
                uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse priority
        let priority: TaskPriority;
        match num_traits::FromPrimitive::from_i32(task_row.priority) {
            Some(parsed_priority) => {
                priority = parsed_priority
            },
            None => {
                priority = TaskPriority::NORMAL
            }
        }
        // Parse tags
        let tags: Option<Vec<String>>;
        match task_row.tags {
            Some(tags_json_str) => {
                match serde_json::from_str::<Vec<String>>(&tags_json_str) {
                    Ok(parsed_tags) => {
                        tags = Some(parsed_tags)
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                tags = None
            }
        }
        // Parse parent list uuid
        let parent_list_uuid: Uuid;
        match Uuid::parse_str(&task_row.parent_list_uuid) {
            Ok(res) => {
                parent_list_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse creation information uuid
        let creation_information_uuid: Uuid;
        match Uuid::parse_str(&task_row.creation_information_uuid) {
            Ok(res) => {
                creation_information_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        Ok(Task {
            uuid,
            content: task_row.content,
            priority,
            tags,
            is_complete: task_row.is_complete,
            parent_list_uuid,
            creation_information_uuid
        })
    }
}
