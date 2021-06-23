use chrono::{DateTime, Utc};
use uuid::Uuid;
use juniper::GraphQLObject;

use crate::api::models::CreationInformationStruct;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct CreationInformationRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub creator_user_uuid: String,
    pub creation_time: String,
    pub last_updated_by_user_uuid: String,
    pub last_updated_time: String
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
