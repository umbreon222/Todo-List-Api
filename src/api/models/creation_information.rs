use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api::models::database::NewCreationInformationRow;

pub struct CreationInformationStruct {
    pub uuid: Uuid,
    pub creator_user_uuid: Uuid,
    pub creation_time: DateTime<Utc>,
    pub last_updated_by_user_uuid: Uuid,
    pub last_updated_time: DateTime<Utc>
}

impl CreationInformationStruct {
    pub fn create_new_creation_information_row(&self) -> NewCreationInformationRow {
        NewCreationInformationRow {
            uuid: self.uuid.to_string(),
            creator_user_uuid: self.creator_user_uuid.to_string(),
            creation_time: self.creation_time.to_rfc3339(),
            last_updated_by_user_uuid: self.last_updated_by_user_uuid.to_string(),
            last_updated_time: self.last_updated_time.to_rfc3339()
        }
    }
}
