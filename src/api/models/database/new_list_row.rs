use crate::api::models::List;
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

impl NewListRow {
    pub fn from_list(list: List) -> NewListRow {
        NewListRow {
            uuid: list.uuid.to_string(),
            title: list.title,
            description: list.description,
            color_hex: list.color_hex,
            creation_information_uuid: list.creation_information_uuid.to_string()
        }
    }
}
