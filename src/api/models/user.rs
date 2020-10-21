use juniper::{GraphQLInputObject, GraphQLObject};

use crate::api::schema::*;

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
