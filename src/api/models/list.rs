use uuid::Uuid;

use crate::api::models::database::NewListRow;

pub struct List {
    pub uuid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<Vec<Uuid>>,
    pub parent_list_uuid: Option<Uuid>,
    pub sub_list_uuids: Option<Vec<Uuid>>,
    pub shared_with_user_uuids: Option<Vec<Uuid>>,
    pub creation_information_uuid: Uuid
}

impl List {
    pub fn create_new_list_row(&self) -> Result<NewListRow, String> {
        // Convert task uuids to json
        let json_task_uuids: Option<String>;
        match &self.task_uuids {
            Some(task_uuids) => {
                let string_task_uuids: Vec<String> = task_uuids
                    .into_iter()
                    .map(|uuid| uuid.to_string())
                    .collect();
                match serde_json::to_string(&string_task_uuids) {
                    Ok(res) => {
                        json_task_uuids = Some(res);
                    },
                    Err(_) => {
                        return Err(String::from("Error serializing task uuids to json"));
                    }
                }
            },
            None => {
                json_task_uuids = None;
            }
        }
        // Convert parent list uuid to string
        let string_parent_list_uuid: Option<String>;
        match &self.parent_list_uuid {
            Some(parent_list_uuid) => {
                string_parent_list_uuid = Some(parent_list_uuid.to_string());
            },
            None => {
                string_parent_list_uuid = None;
            }
        }
        // Convert sub list uuids to json
        let json_sub_list_uuids: Option<String>;
        match &self.sub_list_uuids {
            Some(sub_list_uuids) => {
                let string_sub_list_uuids: Vec<String> = sub_list_uuids
                    .into_iter()
                    .map(|uuid: &Uuid| uuid.to_string())
                    .collect();
                match serde_json::to_string(&string_sub_list_uuids) {
                    Ok(res) => {
                        json_sub_list_uuids = Some(res);
                    },
                    Err(_) => {
                        return Err(String::from("Error serializing sub list uuids to json"));
                    }
                }
            },
            None => {
                json_sub_list_uuids = None;
            }
        }
        // Convert shared with uuids to json
        let json_shared_with_uuids: Option<String>;
        match &self.shared_with_user_uuids {
            Some(shared_with_user_uuids) => {
                let string_shared_with_user_uuids: Vec<String> = shared_with_user_uuids
                    .into_iter()
                    .map(|uuid| uuid.to_string())
                    .collect();
                match serde_json::to_string(&string_shared_with_user_uuids) {
                    Ok(res) => {
                        json_shared_with_uuids = Some(res);
                    },
                    Err(_) => {
                        return Err(String::from("Error serializing shared with uuids to json"));
                    }
                }
            },
            None => {
                json_shared_with_uuids = None;
            }
        }
        Ok(NewListRow {
            uuid: self.uuid.to_string(),
            title: self.title.clone(),
            description: self.description.clone(),
            color_hex: self.color_hex.clone(),
            task_uuids: json_task_uuids,
            parent_list_uuid: string_parent_list_uuid,
            sub_list_uuids: json_sub_list_uuids,
            shared_with_user_uuids: json_shared_with_uuids,
            creation_information_uuid: self.creation_information_uuid.to_string()
        })
    } 
}
