use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants::{ERROR_DETAILS_KEY, USER_NOT_CREATED_ERROR_MESSAGE, INTERNAL_ERROR};
use crate::api::{models, schema};
use schema::Users::dsl::*;
use crate::api::services::utilities::graphql_translate;

pub struct UserService;

impl UserService {
    pub fn all_users(conn: &SqliteConnection) -> FieldResult<Vec<models::UserRow>> {
        graphql_translate(Users.load::<models::UserRow>(conn))
    }

    pub fn create_user(
        conn: &SqliteConnection,
        create_user_input: models::CreateUserInput
    ) -> FieldResult<models::UserRow> {
        // Parse create user input
        let new_user = create_user_input.create_user();
        // Create new user row
        let new_user_row = new_user.create_new_user_row();
        // Execute insertion
        match diesel::insert_into(schema::Users::table).values(&new_user_row).execute(conn) {
            Ok(_) => {},
            Err(err) => {
                let error_details = err.to_string();
                return Err(FieldError::new(USER_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
            }
        }
        // Return error or newly inserted row via UUID look up
        match UserService::get_user_by_uuid(&conn, &new_user.uuid.to_string()) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!("Couldn't find user '{}' after insert", new_user.uuid.to_string());
                        Err(FieldError::new(INTERNAL_ERROR, graphql_value!({ ERROR_DETAILS_KEY: error_details })))
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_user_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::UserRow>> {
        match Users.filter(UUID.eq(uuid.clone())).first::<models::UserRow>(conn) {
            Ok(user) => Ok(Some(user)),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(FieldError::from(err))
            }
        }
    }

    pub fn user_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Users.filter(UUID.eq(uuid.clone())))).get_result::<bool>(conn));
    }
}
