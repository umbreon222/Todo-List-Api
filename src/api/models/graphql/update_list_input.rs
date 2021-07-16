use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::models::List;
use crate::api::models::utilities::{parse_color_hex, parse_json_uuid_array};

#[derive(GraphQLInputObject)]
pub struct UpdateListInput {
    pub uuid: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>
}

impl UpdateListInput {
    pub fn create_updated_list(&self, list: List) -> Result<List, String> {
        let title = self.title.clone().unwrap_or(list.title);
        // Update description
        let description: Option<String>;
        match &self.description {
            Some(res) => {
                description = Some(res.to_string());
            }
            None => {
                description = list.description;
            }
        }
        // Update color hex
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
                color_hex = list.color_hex;
            }
        }
        // Update task uuids
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
                task_uuids = list.task_uuids;
            }
        }
        // Update parent list uuid
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
                parent_list_uuid = list.parent_list_uuid;
            }
        }
        // Update sub list uuids
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
                sub_list_uuids = list.sub_list_uuids;
            }
        }
        // Update shared with user uuids
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
                shared_with_user_uuids = list.shared_with_user_uuids;
            }
        }
        Ok(List {
            uuid: list.uuid.clone(),
            title,
            description,
            color_hex,
            task_uuids,
            parent_list_uuid,
            sub_list_uuids,
            shared_with_user_uuids,
            creation_information_uuid: list.creation_information_uuid.clone()
        })
    }
}
