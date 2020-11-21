use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;
use crate::api::errors::ValidationError;
use crate::api::validators::*;

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

impl CreateCreationInformationInput {
    pub fn validate(&self) -> Result<(), ValidationError> {
        match validate_uuid(&self.creator_user_uuid) {
            Ok(_) => Ok(()),
            Err(err) => return Err(err),
        }
    }
}