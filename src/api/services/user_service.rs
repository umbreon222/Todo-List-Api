use uuid::Uuid;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::services::utilities::graphql_translate;
use crate::api::schema;
use crate::api::models;

const PASSWORD_HASH_SALT: &'static str = "pr3tz3ls&mcd0nalds_fr1es";

pub struct UserService;

impl UserService {
    pub fn all_users(conn: &SqliteConnection) -> FieldResult<Vec<models::User>> {
        use schema::Users::dsl::*;

        graphql_translate(Users.load::<models::User>(conn))
    }

    pub fn create_user(
        conn: &SqliteConnection,
        new_user: models::CreateUserInput,
    ) -> FieldResult<models::User> {
        use schema::Users::dsl::*;

        // Create new user row
        let uuid = Uuid::new_v4();
        // This may need to be done client side to avoid sending the real user's password over the network
        let mut salted_password = new_user.password.clone();
        salted_password.push_str(PASSWORD_HASH_SALT);
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(&salted_password);
        let password_hash = hasher.result_str();
        let new_user = models::NewUser {
            UUID: &uuid.to_string(),
            Username: &new_user.username,
            PasswordHash: &password_hash,
            Nickname: &new_user.nickname
        };
        // Execute insertion
        let inserted = diesel::insert_into(schema::Users::table)
            .values(&new_user)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Users.filter(UUID.eq(uuid.to_string())).first::<models::User>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("User not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_user_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<models::User>> {
        use schema::Users::dsl::*;

        match Users.filter(UUID.eq(uuid)).first::<models::User>(conn) {
            Ok(user) => Ok(Some(user)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }

    pub fn user_exists(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<bool> {
        use schema::Users::dsl::*;
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Users.filter(UUID.eq(uuid)))).get_result::<bool>(conn));
    }
}
