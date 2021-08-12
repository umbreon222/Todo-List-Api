use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api::models::database::CreationInformationRow;
use crate::api::models::graphql::CreateCreationInformationInput;

pub struct CreationInformation {
    pub uuid: Uuid,
    pub creator_user_uuid: Uuid,
    pub creation_time: DateTime<Utc>,
    pub last_updated_by_user_uuid: Uuid,
    pub last_updated_time: DateTime<Utc>
}

impl CreationInformation {
    pub fn from_creation_information_row(creation_information_row: CreationInformationRow) -> Result<CreationInformation, String> {
        // Parse uuid
        let uuid: Uuid;
        match Uuid::parse_str(&creation_information_row.uuid) {
            Ok(res) => {
                uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse creator user uuid
        let creator_user_uuid: Uuid;
        match Uuid::parse_str(&creation_information_row.creator_user_uuid) {
            Ok(res) => {
                creator_user_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse creation time
        let creation_time: DateTime<Utc>;
        match DateTime::parse_from_rfc3339(&creation_information_row.creation_time) {
            Ok(res) => {
                creation_time = res.with_timezone(&Utc);
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse last updated by user uuid
        let last_updated_by_user_uuid: Uuid;
        match Uuid::parse_str(&creation_information_row.last_updated_by_user_uuid) {
            Ok(res) => {
                last_updated_by_user_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse last updated time
        let last_updated_time: DateTime<Utc>;
        match DateTime::parse_from_rfc3339(&creation_information_row.last_updated_time) {
            Ok(res) => {
                last_updated_time = res.with_timezone(&Utc);
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        Ok(CreationInformation {
            uuid,
            creator_user_uuid,
            creation_time,
            last_updated_by_user_uuid,
            last_updated_time
        })
    }

    pub fn from_create_creation_information_input(input: CreateCreationInformationInput) -> Result<CreationInformation, String> {
        // Generate uuid for new creation information
        let uuid = Uuid::new_v4();
        // Parse creator user uuid
        match Uuid::parse_str(&input.creator_user_uuid) {
            Ok(creator_user_uuid) => {
                let current_time = Utc::now();
                Ok(CreationInformation {
                    uuid,
                    creator_user_uuid: creator_user_uuid,
                    creation_time: current_time.clone(),
                    last_updated_by_user_uuid: creator_user_uuid,
                    last_updated_time: current_time
                })
            },
            Err(err) => Err(err.to_string())
        }
    }
    
    pub fn set_last_updated(&mut self, updated_by_user_uuid: Uuid) {
        self.last_updated_by_user_uuid = updated_by_user_uuid;
        self.last_updated_time = Utc::now();
    }
}
