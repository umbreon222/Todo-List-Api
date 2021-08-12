use juniper::GraphQLObject;

use crate::api::models::List;
use crate::api::schema::*;

#[derive(GraphQLObject, Queryable, Insertable)]
#[table_name = "lists"]
pub struct ListRow {
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
    pub fn from_list(list: List) -> Result<ListRow, String> {
        // Serialize task uuids to json
        let task_uuids: Option<String>;
        match list.task_uuids {
            Some(uuids) => {
                let task_uuid_strings: Vec<String> = uuids.iter().map(|uuid| uuid.to_string()).collect();
                match serde_json::to_string(&task_uuid_strings) {
                    Ok(res) => {
                        task_uuids = Some(res);
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
        // Serialize parent list uuid to json
        let parent_list_uuid: Option<String>;
        match list.parent_list_uuid {
            Some(uuid) => {
                parent_list_uuid = Some(uuid.to_string());
            },
            None => {
                parent_list_uuid = None;
            }
        }
        // Serialize sub list uuids to json
        let sub_list_uuids: Option<String>;
        match list.sub_list_uuids {
            Some(uuids) => {
                let sub_list_uuid_strings: Vec<String> = uuids.iter().map(|uuid| uuid.to_string()).collect();
                match serde_json::to_string(&sub_list_uuid_strings) {
                    Ok(res) => {
                        sub_list_uuids = Some(res);
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
        // Serialize shared with user uuids to json
        let shared_with_user_uuids: Option<String>;
        match list.shared_with_user_uuids {
            Some(uuids) => {
                let shared_with_user_uuid_strings: Vec<String> = uuids.iter().map(|uuid| uuid.to_string()).collect();
                match serde_json::to_string(&shared_with_user_uuid_strings) {
                    Ok(res) => {
                        shared_with_user_uuids = Some(res);
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
        Ok(ListRow {
            uuid: list.uuid.to_string(),
            title: list.title,
            description: list.description,
            color_hex: list.color_hex,
            task_uuids,
            parent_list_uuid,
            sub_list_uuids,
            shared_with_user_uuids,
            creation_information_uuid: list.creation_information_uuid.to_string()
        })
    }
}
