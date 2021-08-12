use uuid::Uuid;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use std::env;

use crate::api::models::graphql::CreateUserInput;

pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}

impl User {
    pub fn from_create_user_input(input: CreateUserInput) -> User {
        // Generate uuid for new user
        let uuid = Uuid::new_v4();
        let mut salted_password = input.password.clone();
        // TODO: handle errors
        let hash_salt = env::var("PASSWORD_HASH_SALT").expect("no password hash salt");
        salted_password.push_str(&hash_salt);
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(&salted_password);
        let password_hash = hasher.result_str();
        return User {
            uuid,
            username: input.username,
            password_hash,
            nickname: input.nickname
        }
    }
}
