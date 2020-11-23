use uuid::Uuid;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;
use crate::api::models::utilities::{parse_color_hex, parse_json_uuid_array};

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct ListRow {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Title")]
    pub title: String,
    #[graphql(name = "Description")]
    pub description: Option<String>,
    #[graphql(name = "ColorHex")]
    pub color_hex: Option<String>,
    #[graphql(name = "TaskUUIDs")]
    pub task_uuids: Option<String>,
    #[graphql(name = "ParentListUUID")]
    pub parent_list_uuid: Option<String>,
    #[graphql(name = "SubListUUIDs")]
    pub sub_list_uuids: Option<String>,
    #[graphql(name = "SharedWithUserUUIDs")]
    pub shared_with_user_uuids: Option<String>,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: String
}

#[derive(Insertable)]
#[table_name = "Lists"]
#[allow(non_snake_case)]
pub struct NewListRow {
    pub UUID: String,
    pub Title: String,
    pub Description: Option<String>,
    pub ColorHex: Option<String>,
    pub TaskUUIDs: Option<String>,
    pub ParentListUUID: Option<String>,
    pub SubListUUIDs: Option<String>,
    pub SharedWithUserUUIDs: Option<String>,
    pub CreationInformationUUID: String
}

#[derive(GraphQLInputObject)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>
}

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
        }
        Ok(List {
            uuid,
            title: self.title.clone(),
            description: self.description.clone(),
            color_hex,
            task_uuids,
            parent_list_uuid,
            sub_list_uuids,
            shared_with_user_uuids,
            creation_information_uuid: creation_information_uuid.clone()
        })
    }
}

impl List {
    pub fn create_new_list_row(&self) -> Result<NewListRow, String> {
        // Convert task uuids to json
        let json_task_uuids: Option<String>;
        match &self.task_uuids {
            Some(task_uuids) => {
                let string_task_uuids: Vec<String> = task_uuids.into_iter().map(|uuid| uuid.to_string()).collect();
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
                let string_sub_list_uuids: Vec<String> = sub_list_uuids.into_iter().map(|uuid| uuid.to_string()).collect();
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
                let string_shared_with_user_uuids: Vec<String> = shared_with_user_uuids.into_iter().map(|uuid| uuid.to_string()).collect();
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
            UUID: self.uuid.to_string(),
            Title: self.title.clone(),
            Description: self.description.clone(),
            ColorHex: self.color_hex.clone(),
            TaskUUIDs: json_task_uuids,
            ParentListUUID: string_parent_list_uuid,
            SubListUUIDs: json_sub_list_uuids,
            SharedWithUserUUIDs: json_shared_with_uuids,
            CreationInformationUUID: self.creation_information_uuid.to_string()
        })
    } 
}
