use juniper::{GraphQLInputObject, GraphQLObject};

use super::schema::*;

// User
#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct User {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Username")]
    pub username: String,
    #[graphql(name = "PasswordHash")]
    pub password_hash: String,
    #[graphql(name = "Nickname")]
    pub nickname: String
}

#[derive(Insertable)]
#[table_name = "Users"]
#[allow(non_snake_case)]
pub struct NewUser<'a> {
    pub UUID: &'a String,
    pub Username: &'a String,
    pub PasswordHash: &'a String,
    pub Nickname: &'a String
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub nickname: String
}

// CreationInformation
#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct CreationInformationStruct {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "CreatorUserUUID")]
    pub creator_user_uuid: String,
    #[graphql(name = "CreationTime")]
    pub creation_time: String,
    #[graphql(name = "LastUpdatedByUserUUID")]
    pub last_updated_by_user_uuid: String,
    #[graphql(name = "LastUpdatedTime")]
    pub last_updated_time: String
}

#[derive(Insertable)]
#[table_name = "CreationInformation"]
#[allow(non_snake_case)]
pub struct NewCreationInformation<'a> {
    pub UUID: &'a String,
    pub CreatorUserUUID: &'a String,
    pub CreationTime: &'a String,
    pub LastUpdatedByUserUUID: &'a String,
    pub LastUpdatedTime: &'a String
}

#[derive(GraphQLInputObject)]
pub struct CreateCreationInformationInput {
    pub creator_user_uuid: String,
}

// List
#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct List {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Title")]
    pub title: String,
    #[graphql(name = "Description")]
    pub description: Option<String>,
    #[graphql(name = "ColorHex")]
    pub color_hex: Option<String>,
    #[graphql(name = "TaskUUIDs")]
    pub task_uuids: Option<String>,
    #[graphql(name = "ParentListUUID")]
    pub parent_list_uuid: Option<String>,
    #[graphql(name = "SubListUUIDs")]
    pub sub_list_uuids: Option<String>,
    #[graphql(name = "SharedWithUserUUIDs")]
    pub shared_with_user_uuids: Option<String>,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: String
}

#[derive(Insertable)]
#[table_name = "Lists"]
#[allow(non_snake_case)]
pub struct NewList<'a> {
    pub UUID: &'a String,
    pub Title: &'a String,
    pub Description: Option<String>,
    pub ColorHex: Option<String>,
    pub TaskUUIDs: Option<String>,
    pub ParentListUUID: Option<String>,
    pub SubListUUIDs: Option<String>,
    pub SharedWithUserUUIDs: Option<String>,
    pub CreationInformationUUID: &'a String
}

#[derive(GraphQLInputObject)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>
}

// Task
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

// Tag
#[derive(GraphQLObject)]
struct Tag {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Title")]
    pub title: String,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: i32
}
