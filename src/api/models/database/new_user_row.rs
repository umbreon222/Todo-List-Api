use crate::api::schema::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserRow {
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}
