use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct Task {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Content")]
    pub content: String,
    #[graphql(name = "Priority")]
    pub priority: i32,
    #[graphql(name = "TagUUIDs")]
    pub tag_uuids: Option<String>,
    #[graphql(name = "IsComplete")]
    pub is_complete: bool,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: String
}

#[derive(Insertable)]
#[table_name = "Tasks"]
#[allow(non_snake_case)]
pub struct NewTask<'a> {
    pub UUID: &'a String,
    pub Content: &'a String,
    pub Priority: i32,
    pub TagUUIDs: Option<String>,
    pub IsComplete: bool,
    pub CreationInformationUUID: &'a String
}

#[derive(GraphQLInputObject)]
pub struct CreateTaskInput {
    pub content: String,
    pub priority: Option<i32>,
    pub tag_uuids: Option<String>,
    pub is_complete: Option<bool>,
}
