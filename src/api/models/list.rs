use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;
use crate::api::errors::ValidationError;
use crate::api::validators::*;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct List {
    #[graphql(name = "ID")]
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
pub struct NewList<'a> {
    pub UUID: &'a String,
    pub Title: &'a String,
    pub Description: Option<String>,
    pub ColorHex: Option<String>,
    pub TaskUUIDs: Option<String>,
    pub ParentListUUID: Option<String>,
    pub SubListUUIDs: Option<String>,
    pub SharedWithUserUUIDs: Option<String>,
    pub CreationInformationUUID: &'a String
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

impl CreateListInput {
    pub fn validate(&self) -> Result<(), ValidationError> {
        match &self.color_hex {
            Some(color_hex) => {
                match validate_color_hex(color_hex) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            },
            None => {},
        }
        match &self.task_uuids {
            Some(task_uuids_json) => {
                match validate_json_uuid_array(task_uuids_json) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            },
            None => {},
        }
        match &self.parent_list_uuid {
            Some(parent_list_uuid) => {
                match validate_uuid(parent_list_uuid) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            },
            None => {},
        }
        match &self.sub_list_uuids {
            Some(sub_list_uuids_json) => {
                match validate_json_uuid_array(sub_list_uuids_json) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
            },
            None => {},
        }
        match &self.shared_with_user_uuids {
            Some(shared_with_user_uuids_json) => {
                validate_json_uuid_array(shared_with_user_uuids_json)
            },
            None => Ok(()),
        }
    }
}
