use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::models::List;
use crate::api::models::utilities::parse_color_hex;

#[derive(GraphQLInputObject)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
}

impl CreateListInput {
    pub fn create_list(&self, creation_information_uuid: &Uuid) -> Result<List, String> {
        // Generate uuid for new list
        let uuid = Uuid::new_v4();
        // Parse color hex
        let color_hex: Option<String>;
        match &self.color_hex {
            Some(string_color_hex) => {
                match parse_color_hex(&string_color_hex) {
                    Ok(res) => {
                        color_hex = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                color_hex = None;
            }
        }
        // These make this exponentially large and should be broken up
        /*
        // Parse task uuids
        let task_uuids: Option<Vec<Uuid>>;
        match &self.task_uuids {
            Some(task_uuids_json) => {
                match parse_json_uuid_array(&task_uuids_json) {
                    Ok(res) => {
                        task_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                task_uuids = None;
            }
        }
        // Parse parent list uuid
        let parent_list_uuid: Option<Uuid>;
        match &self.parent_list_uuid {
            Some(string_parent_list_uuid) => {
                match Uuid::parse_str(&string_parent_list_uuid) {
                    Ok(res) => {
                        parent_list_uuid = Some(res);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                parent_list_uuid = None;
            }
        }
        // Parse sub list uuids
        let sub_list_uuids: Option<Vec<Uuid>>;
        match &self.sub_list_uuids {
            Some(sub_list_uuids_json) => {
                match parse_json_uuid_array(&sub_list_uuids_json) {
                    Ok(res) => {
                        sub_list_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                sub_list_uuids = None;
            }
        }
        // Parse shared with user uuids
        let shared_with_user_uuids: Option<Vec<Uuid>>;
        match &self.shared_with_user_uuids {
            Some(shared_with_user_uuids_json) => {
                match parse_json_uuid_array(&shared_with_user_uuids_json) {
                    Ok(res) => {
                        shared_with_user_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                shared_with_user_uuids = None;
            }
        }*/
        Ok(List {
            uuid,
            title: self.title.clone(),
            description: self.description.clone(),
            color_hex,
            task_uuids: None,
            parent_list_uuid: None,
            sub_list_uuids: None,
            shared_with_user_uuids: None,
            creation_information_uuid: creation_information_uuid.clone()
        })
    }
}
