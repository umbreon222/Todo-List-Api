use uuid::Uuid;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use std::env;
use juniper::GraphQLInputObject;

use crate::api::models::User;

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub nickname: String
}

impl CreateUserInput {
    pub fn create_user(&self) -> User {
        // Generate uuid for new user
        let uuid = Uuid::new_v4();
        let mut salted_password = self.password.clone();
        // TODO: handle errors
        let hash_salt = env::var("PASSWORD_HASH_SALT").expect("no password hash salt");
        salted_password.push_str(&hash_salt);
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
