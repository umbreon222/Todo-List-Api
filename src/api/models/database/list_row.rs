use uuid::Uuid;
use juniper::GraphQLObject;

use crate::api::models::List;
use crate::api::models::utilities::{parse_color_hex, parse_json_uuid_array};

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct ListRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>,
    pub creation_information_uuid: String
}

impl ListRow {
    pub fn create_list(&self) -> Result<List, String> {
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
        // Parse color hex
        let color_hex: Option<String>;
        match &self.color_hex {
            Some(res) => {
                match parse_color_hex(&res) {
                    Ok(parsed_color_hex) => {
                        color_hex = Some(parsed_color_hex);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                color_hex = None;
            }
        }
        // Parse task uuids
        let task_uuids: Option<Vec<Uuid>>;
        match &self.task_uuids {
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_task_uuids) => {
                        task_uuids = Some(parsed_task_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
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
            Some(res) => {
                match Uuid::parse_str(&res) {
                    Ok(parsed_parent_list_uuid) => {
                        parent_list_uuid = Some(parsed_parent_list_uuid);
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
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_sub_list_uuids) => {
                        sub_list_uuids = Some(parsed_sub_list_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
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
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_shared_with_user_uuids) => {
                        shared_with_user_uuids = Some(parsed_shared_with_user_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                shared_with_user_uuids = None;
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
        Ok(List {
            uuid,
            title: self.title.to_string(),
            description: self.description.clone(),
            color_hex,
            task_uuids,
            parent_list_uuid,
            sub_list_uuids,
            shared_with_user_uuids,
            creation_information_uuid
        })
    }
}