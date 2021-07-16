use uuid::Uuid;

use crate::api::models::database::NewTaskRow;

pub struct Task {
    pub uuid: Uuid,
    pub content: String,
    pub priority: i32,
    pub tags: Option<Vec<String>>,
    pub is_complete: bool,
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
        Ok(NewTaskRow {
            uuid: self.uuid.to_string(),
            content: self.content.clone(),
            priority: self.priority,
            tags: json_tags,
            is_complete: self.is_complete,
            creation_information_uuid: self.creation_information_uuid.to_string()
        })
    } 
}
