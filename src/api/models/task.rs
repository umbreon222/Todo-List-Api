use uuid::Uuid;

use crate::api::models::database::NewTaskRow;

pub struct Task {
    pub uuid: Uuid,
    pub content: String,
    pub priority: i32,
    pub tag_uuids: Option<Vec<Uuid>>,
    pub is_complete: bool,
    pub creation_information_uuid: Uuid
}

impl Task {
    pub fn create_new_task_row(&self) -> Result<NewTaskRow, String> {
        // Convert tag uuids to json
        let json_tag_uuids: Option<String>;
        match &self.tag_uuids {
            Some(tag_uuids) => {
                let string_tag_uuids: Vec<String> = tag_uuids
                    .into_iter()
                    .map(|uuid| uuid.to_string())
                    .collect();
                match serde_json::to_string(&string_tag_uuids) {
                    Ok(res) => {
                        json_tag_uuids = Some(res);
                    },
                    Err(_) => {
                        return Err(String::from("Error serializing tag uuids to json"));
                    }
                }
            },
            None => {
                json_tag_uuids = None;
            }
        }
        Ok(NewTaskRow {
            uuid: self.uuid.to_string(),
            content: self.content.clone(),
            priority: self.priority,
            tag_uuids: json_tag_uuids,
            is_complete: self.is_complete,
            creation_information_uuid: self.creation_information_uuid.to_string()
        })
    } 
}
