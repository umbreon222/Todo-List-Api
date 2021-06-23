use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::users::dsl;
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct UserService;

impl UserService {
    pub fn all_users(conn: &SqliteConnection) -> FieldResult<Vec<models::database::UserRow>> {
        graphql_translate(dsl::users.load::<models::database::UserRow>(conn))
    }

    pub fn create_user(
        conn: &SqliteConnection,
        create_user_input: models::graphql::CreateUserInput
    ) -> FieldResult<models::database::UserRow> {
        // Parse create user input
        let new_user = create_user_input.create_user();
        // Create new user row
        let new_user_row = new_user.create_new_user_row();
        // Execute insertion
        match diesel::insert_into(schema::users::table)
            .values(&new_user_row)
            .execute(conn) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::USER_NOT_CREATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Return error or newly inserted row via UUID look up
        match UserService::get_user_by_uuid(&conn, &new_user.uuid.to_string()) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find user '{}' after insert",
                            new_user.uuid.to_string()
                        );
                        Err(
                            graphql_error_translate(
                                constants::USER_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                error_details)
                        )
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_user_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::database::UserRow>> {
        match dsl::users.filter(dsl::uuid.eq(uuid.clone())).first::<models::database::UserRow>(conn) {
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
        
        return graphql_translate(
            select(
                exists(dsl::users.filter(dsl::uuid.eq(uuid.clone())))
            ).get_result::<bool>(conn)
        );
    }
}
