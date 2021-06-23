use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "lists"]
pub struct NewListRow {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>,
    pub creation_information_uuid: String
}
