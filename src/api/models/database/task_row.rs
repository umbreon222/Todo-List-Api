use uuid::Uuid;
use juniper::GraphQLObject;

use crate::api::models::{Task, TaskPriority};

#[derive(GraphQLObject)]
#[derive(Queryable)]
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
    pub fn create_task(&self) -> Result<Task, String> {
        // Parse uuid
        let uuid: Uuid;
        match Uuid::parse_str(&self.uuid) {
            Ok(res) => {
                uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse priority
        let priority: TaskPriority;
        match num_traits::FromPrimitive::from_i32(self.priority) {
            Some(parsed_priority) => {
                priority = parsed_priority
            },
            None => {
                priority = TaskPriority::NORMAL
            }
        }
        // Parse tags
        let tags: Option<Vec<String>>;
        match &self.tags {
            Some(tags_json_str) => {
                match serde_json::from_str::<Vec<String>>(tags_json_str) {
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
        match Uuid::parse_str(&self.parent_list_uuid) {
            Ok(res) => {
                parent_list_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse creation information uuid
        let creation_information_uuid: Uuid;
        match Uuid::parse_str(&self.creation_information_uuid) {
            Ok(res) => {
                creation_information_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        Ok(Task {
            uuid,
            content: self.content.clone(),
            priority,
            tags,
            is_complete: self.is_complete,
            parent_list_uuid,
            creation_information_uuid
        })
    }
}