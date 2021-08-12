use juniper::GraphQLObject;

use crate::api::models::User;
use crate::api::schema::*;

#[derive(GraphQLObject, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserRow {
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}

impl UserRow {
    pub fn from_user(user: User) -> UserRow {
        UserRow {
            uuid: user.uuid.to_string(),
            username: user.username,
            password_hash: user.password_hash,
            nickname: user.nickname
        }
    }
}
