use uuid::Uuid;

use crate::api::models::database::NewTaskRow;

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
    pub fn create_new_task_row(&self) -> Result<NewTaskRow, String> {
        // Convert tags to json
        let json_tags: Option<String>;
        match &self.tags {
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
        let priority = num_traits::ToPrimitive::to_i32(&self.priority);
        Ok(NewTaskRow {
            uuid: self.uuid.to_string(),
            content: self.content.clone(),
            priority: priority.unwrap_or(1), // Default to normal
            tags: json_tags,
            is_complete: self.is_complete,
            parent_list_uuid: self.parent_list_uuid.to_string(),
            creation_information_uuid: self.creation_information_uuid.to_string()
        })
    } 
}
