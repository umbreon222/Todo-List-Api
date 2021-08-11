use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::api::models::database::{CreationInformationRow, NewCreationInformationRow};

pub struct CreationInformation {
    pub uuid: Uuid,
    pub creator_user_uuid: Uuid,
    pub creation_time: DateTime<Utc>,
    pub last_updated_by_user_uuid: Uuid,
    pub last_updated_time: DateTime<Utc>
}

impl CreationInformation {
    pub fn create_new_creation_information_row(&self) -> NewCreationInformationRow {
        NewCreationInformationRow {
            uuid: self.uuid.to_string(),
            creator_user_uuid: self.creator_user_uuid.to_string(),
            creation_time: self.creation_time.to_rfc3339(),
            last_updated_by_user_uuid: self.last_updated_by_user_uuid.to_string(),
            last_updated_time: self.last_updated_time.to_rfc3339()
        }
    }

    pub fn create_updated_creation_information_row(
        &self,
        creation_information_row: CreationInformationRow
    ) -> CreationInformationRow {
        // We can cheat and use the above function to do the conversion for us
        let new_creation_information_row = self.create_new_creation_information_row();
        CreationInformationRow {
            uuid: new_creation_information_row.uuid,
            creator_user_uuid: new_creation_information_row.creator_user_uuid,
            creation_time: new_creation_information_row.creation_time,
            last_updated_by_user_uuid: creation_information_row.last_updated_by_user_uuid,
            last_updated_time: new_creation_information_row.last_updated_time
        }
    }
}
