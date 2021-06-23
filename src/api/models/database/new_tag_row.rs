use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTagRow {
    pub uuid: String,
    pub title: String,
    pub creation_information_uuid: String
}