use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "creation_information"]
pub struct NewCreationInformationRow {
    pub uuid: String,
    pub creator_user_uuid: String,
    pub creation_time: String,
    pub last_updated_by_user_uuid: String,
    pub last_updated_time: String
}
