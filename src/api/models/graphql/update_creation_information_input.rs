use chrono::Utc;
use uuid::Uuid;
use juniper::GraphQLInputObject;

use crate::api::models::CreationInformation;

#[derive(GraphQLInputObject)]
pub struct UpdateCreationInformationInput {
    pub last_updated_by_user_uuid: String
}

impl UpdateCreationInformationInput {
    pub fn create_updated_creation_information(&self, creation_information: CreationInformation) -> Result<CreationInformation, String> {
        // Parse last updated by user uuid
        match Uuid::parse_str(&self.last_updated_by_user_uuid) {
            Ok(last_updated_by_user_uuid) => {
                let current_time = Utc::now();
                Ok(CreationInformation {
                    uuid: creation_information.uuid,
                    creator_user_uuid: creation_information.creator_user_uuid,
                    creation_time: creation_information.creation_time,
                    last_updated_by_user_uuid,
                    last_updated_time: current_time
                })
            },
            Err(err) => Err(err.to_string())
        }
    }
}
