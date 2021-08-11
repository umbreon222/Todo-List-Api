
use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::models::{Task, TaskPriority};

#[derive(GraphQLInputObject)]
pub struct CreateTaskInput {
    pub content: String,
    pub priority: Option<i32>,
    pub tags: Option<String>,
    pub parent_list_uuid: String,
    pub is_complete: Option<bool>,
}

impl CreateTaskInput {
    pub fn create_task(&self, creation_information_uuid: &Uuid) -> Result<Task, String> {
        // Generate uuid for new list
        let uuid = Uuid::new_v4();
        // Parse tags
        let tags: Option<Vec<String>>;
        match &self.tags {
            Some(tags_json) => {
                match serde_json::from_str::<Vec<String>>(&tags_json) {
                    Ok(parsed_tags) => {
                        tags = Some(parsed_tags);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                tags = None;
            }
        }
        // Parse priority
        let priority: TaskPriority;
        match self.priority {
            Some(primitive_priority) => {
                match num_traits::FromPrimitive::from_i32(primitive_priority) {
                    Some(parsed_priority) => {
                        priority = parsed_priority;
                    },
                    None => {
                        // Couldn't parse priority; default to normal.
                        priority = TaskPriority::NORMAL
                    }
                }
            },
            None => {
                // No priority specified; default to normal.
                priority = TaskPriority::NORMAL;
            }
        }
        // Parse parent list uuid
        let parent_list_uuid: Uuid;
        match Uuid::parse_str(&self.parent_list_uuid) {
            Ok(parsed_uuid) => {
                parent_list_uuid = parsed_uuid;
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
            is_complete: self.is_complete.unwrap_or(false),
            parent_list_uuid,
            creation_information_uuid: creation_information_uuid.clone()
        })
    }
}