use juniper::GraphQLObject;

use crate::api::schema::*;
use crate::api::models::CreationInformation;

#[derive(GraphQLObject, Queryable, Clone, Insertable)]
#[table_name = "creation_information"]
pub struct CreationInformationRow {
    pub uuid: String,
    pub creator_user_uuid: String,
    pub creation_time: String,
    pub last_updated_by_user_uuid: String,
    pub last_updated_time: String
}

impl CreationInformationRow {
    pub fn from_creation_information(creation_information: CreationInformation) -> CreationInformationRow {
        CreationInformationRow {
            uuid: creation_information.uuid.to_string(),
            creator_user_uuid: creation_information.creator_user_uuid.to_string(),
            creation_time: creation_information.creation_time.to_rfc3339(),
            last_updated_by_user_uuid: creation_information.last_updated_by_user_uuid.to_string(),
            last_updated_time: creation_information.last_updated_time.to_rfc3339()
        }
    }
}
