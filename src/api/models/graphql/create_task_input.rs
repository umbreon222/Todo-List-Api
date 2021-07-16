use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::constants::LOW_TASK_PRIORITY;
use crate::api::models::Task;

#[derive(GraphQLInputObject)]
pub struct CreateTaskInput {
    pub content: String,
    pub priority: Option<i32>,
    pub tags: Option<String>,
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
        Ok(Task {
            uuid,
            content: self.content.clone(),
            priority: self.priority.unwrap_or(LOW_TASK_PRIORITY),
            tags,
            is_complete: self.is_complete.unwrap_or(false),
            creation_information_uuid: creation_information_uuid.clone()
        })
    }
}