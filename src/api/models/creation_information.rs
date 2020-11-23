use chrono::{DateTime, Utc};
use uuid::Uuid;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct CreationInformationRow {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "CreatorUserUUID")]
    pub creator_user_uuid: String,
    #[graphql(name = "CreationTime")]
    pub creation_time: String,
    #[graphql(name = "LastUpdatedByUserUUID")]
    pub last_updated_by_user_uuid: String,
    #[graphql(name = "LastUpdatedTime")]
    pub last_updated_time: String
}

#[derive(Insertable)]
#[table_name = "CreationInformation"]
#[allow(non_snake_case)]
pub struct NewCreationInformationRow {
    pub UUID: String,
    pub CreatorUserUUID: String,
    pub CreationTime: String,
    pub LastUpdatedByUserUUID: String,
    pub LastUpdatedTime: String
}

#[derive(GraphQLInputObject)]
pub struct CreateCreationInformationInput {
    pub creator_user_uuid: String,
}

pub struct CreationInformationStruct {
    pub uuid: Uuid,
    pub creator_user_uuid: Uuid,
    pub creation_time: DateTime<Utc>,
    pub last_updated_by_user_uuid: Uuid,
    pub last_updated_time: DateTime<Utc>
}

impl CreationInformationRow {
    pub fn create_creation_information_struct(&self) -> Result<CreationInformationStruct, String> {
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
        // Parse creator user uuid
        let creator_user_uuid: Uuid;
        match Uuid::parse_str(&self.creator_user_uuid) {
            Ok(res) => {
                creator_user_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse creation time
        let creation_time: DateTime<Utc>;
        match DateTime::parse_from_rfc3339(&self.creation_time) {
            Ok(res) => {
                creation_time = res.with_timezone(&Utc);
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse last updated by user uuid
        let last_updated_by_user_uuid: Uuid;
        match Uuid::parse_str(&self.last_updated_by_user_uuid) {
            Ok(res) => {
                last_updated_by_user_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse last updated time
        let last_updated_time: DateTime<Utc>;
        match DateTime::parse_from_rfc3339(&self.last_updated_time) {
            Ok(res) => {
                last_updated_time = res.with_timezone(&Utc);
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        Ok(CreationInformationStruct {
            uuid,
            creator_user_uuid,
            creation_time,
            last_updated_by_user_uuid,
            last_updated_time
        })
    }
}

impl CreateCreationInformationInput {
    pub fn create_creation_information(&self) -> Result<CreationInformationStruct, String> {
        // Generate uuid for new creation information
        let uuid = Uuid::new_v4();
        // Parse creator user uuid
        match Uuid::parse_str(&self.creator_user_uuid) {
            Ok(creator_user_uuid) => {
                let current_time = Utc::now();
                Ok(CreationInformationStruct {
                    uuid,
                    creator_user_uuid: creator_user_uuid.clone(),
                    creation_time: current_time.clone(),
                    last_updated_by_user_uuid: creator_user_uuid,
                    last_updated_time: current_time
                })
            },
            Err(err) => Err(err.to_string())
        }
    }
}

impl CreationInformationStruct {
    pub fn create_new_creation_information_row(&self) -> NewCreationInformationRow {
        NewCreationInformationRow {
            UUID: self.uuid.to_string(),
            CreatorUserUUID: self.creator_user_uuid.to_string(),
            CreationTime: self.creation_time.to_rfc3339(),
            LastUpdatedByUserUUID: self.last_updated_by_user_uuid.to_string(),
            LastUpdatedTime: self.last_updated_time.to_rfc3339()
        }
    }
}
