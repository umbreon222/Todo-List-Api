use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::constants::LOW_TASK_PRIORITY;
use crate::api::models::Task;
use crate::api::models::utilities::parse_json_uuid_array;

#[derive(GraphQLInputObject)]
pub struct CreateTaskInput {
    pub content: String,
    pub priority: Option<i32>,
    pub tag_uuids: Option<String>,
    pub is_complete: Option<bool>,
}

impl CreateTaskInput {
    pub fn create_task(&self, creation_information_uuid: &Uuid) -> Result<Task, String> {
        // Generate uuid for new list
        let uuid = Uuid::new_v4();
        // Parse tag uuids
        let tag_uuids: Option<Vec<Uuid>>;
        match &self.tag_uuids {
            Some(tag_uuids_json) => {
                match parse_json_uuid_array(&tag_uuids_json) {
                    Ok(uuids) => {
                        tag_uuids = Some(uuids);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                tag_uuids = None;
            }
        }
        Ok(Task {
            uuid,
            content: self.content.clone(),
            priority: self.priority.unwrap_or(LOW_TASK_PRIORITY),
            tag_uuids,
            is_complete: self.is_complete.unwrap_or(false),
            creation_information_uuid: creation_information_uuid.clone()
        })
    }
}