use uuid::Uuid;

use crate::api::models::database::NewUserRow;

pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}

impl User {
    pub fn create_new_user_row(&self) -> NewUserRow {
        return NewUserRow {
            uuid: self.uuid.to_string(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            nickname: self.nickname.clone()
        }
    } 
}
