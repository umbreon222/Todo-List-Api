use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "lists"]
pub struct NewListRow {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub creation_information_uuid: String
}
