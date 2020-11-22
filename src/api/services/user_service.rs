use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants::{ERROR_DETAILS_KEY, USER_NOT_CREATED_ERROR_MESSAGE};
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
        let inserted = diesel::insert_into(schema::Users::table)
            .values(&new_user_row)
            .execute(conn);
        
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Users.filter(UUID.eq(new_user.uuid.to_string())).first::<models::UserRow>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new(USER_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })))
            }
        }
    }

    pub fn get_user_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::UserRow>> {
        match Users.filter(UUID.eq(uuid.clone())).first::<models::UserRow>(conn) {
            Ok(user) => Ok(Some(user)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(err))
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
