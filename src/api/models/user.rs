use crypto::{digest::Digest, sha3::Sha3};
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::api::schema::*;

const PASSWORD_HASH_SALT: &'static str = "pr3tz3ls&mcd0nalds_fr1es";

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct UserRow {
    #[graphql(skip)]
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
pub struct NewUserRow {
    pub UUID: String,
    pub Username: String,
    pub PasswordHash: String,
    pub Nickname: String
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub nickname: String
}

pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}

impl CreateUserInput {
    pub fn create_user(&self) -> User {
        // Generate uuid for new user
        let uuid = Uuid::new_v4();
        // This may need to be done client side to avoid sending the real user's password over the network
        let mut salted_password = self.password.clone();
        salted_password.push_str(PASSWORD_HASH_SALT);
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(&salted_password);
        let password_hash = hasher.result_str();
        return User {
            uuid,
            username: self.username.clone(),
            password_hash,
            nickname: self.nickname.clone()
        }
    }
}

impl User {
    pub fn create_new_user_row(&self) -> NewUserRow {
        return NewUserRow {
            UUID: self.uuid.to_string(),
            Username: self.username.clone(),
            PasswordHash: self.password_hash.clone(),
            Nickname: self.nickname.clone()
        }
    } 
}
