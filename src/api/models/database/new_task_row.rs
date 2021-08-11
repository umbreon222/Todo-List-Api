use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTaskRow {
    pub uuid: String,
    pub content: String,
    pub priority: i32,
    pub tags: Option<String>,
    pub is_complete: bool,
    pub parent_list_uuid: String,
    pub creation_information_uuid: String
}
