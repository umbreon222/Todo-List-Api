use uuid::Uuid;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::constants::LOW_TASK_PRIORITY;
use crate::api::schema::*;
use crate::api::models::utilities::parse_json_uuid_array;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct TaskRow {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Content")]
    pub content: String,
    #[graphql(name = "Priority")]
    pub priority: i32,
    #[graphql(name = "TagUUIDs")]
    pub tag_uuids: Option<String>,
    #[graphql(name = "IsComplete")]
    pub is_complete: bool,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: String
}

#[derive(Insertable)]
#[table_name = "Tasks"]
#[allow(non_snake_case)]
pub struct NewTaskRow {
    pub UUID: String,
    pub Content: String,
    pub Priority: i32,
    pub TagUUIDs: Option<String>,
    pub IsComplete: bool,
    pub CreationInformationUUID: String
}

#[derive(GraphQLInputObject)]
pub struct CreateTaskInput {
    pub content: String,
    pub priority: Option<i32>,
    pub tag_uuids: Option<String>,
    pub is_complete: Option<bool>,
}

pub struct Task {
    pub uuid: Uuid,
    pub content: String,
    pub priority: i32,
    pub tag_uuids: Option<Vec<Uuid>>,
    pub is_complete: bool,
    pub creation_information_uuid: Uuid
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

impl Task {
    pub fn create_new_task_row(&self) -> Result<NewTaskRow, String> {
        // Convert tag uuids to json
        let json_tag_uuids: Option<String>;
        match &self.tag_uuids {
            Some(tag_uuids) => {
                let string_tag_uuids: Vec<String> = tag_uuids.into_iter().map(|uuid| uuid.to_string()).collect();
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
            UUID: self.uuid.to_string(),
            Content: self.content.clone(),
            Priority: self.priority,
            TagUUIDs: json_tag_uuids,
            IsComplete: self.is_complete,
            CreationInformationUUID: self.creation_information_uuid.to_string()
        })
    } 
}
