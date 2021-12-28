use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::users::dsl;
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct UserService<'a> {
    connection: &'a SqliteConnection,
}

impl<'a> UserService<'a> {
    pub fn new(connection: &'a SqliteConnection) -> Self {
        Self { connection }
    }

    pub fn all_users(&self) -> FieldResult<Vec<models::database::UserRow>> {
        graphql_translate(dsl::users.load::<models::database::UserRow>(self.connection))
    }

    pub fn create_user(
        &self,
        create_user_input: models::graphql::CreateUserInput,
    ) -> FieldResult<models::database::UserRow> {
        // Parse create user input
        let new_user = models::User::from_create_user_input(create_user_input);
        // Create new user row
        let new_user_row = models::database::UserRow::from_user(new_user);
        // Execute insertion
        match diesel::insert_into(schema::users::table)
            .values(&new_user_row)
            .execute(self.connection) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::USER_NOT_CREATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Return error or newly inserted row via UUID look up
        match self.get_user_by_uuid(&new_user_row.uuid) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find user '{}' after insert",
                            &new_user_row.uuid
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
        &self,
        uuid: &String
    ) -> FieldResult<Option<models::database::UserRow>> {
        match dsl::users.filter(dsl::uuid.eq(uuid)).first::<models::database::UserRow>(self.connection) {
            Ok(user) => Ok(Some(user)),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(FieldError::from(err))
            }
        }
    }

    pub fn user_exists(
        &self,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(
            select(
                exists(dsl::users.filter(dsl::uuid.eq(uuid)))
            ).get_result::<bool>(self.connection)
        );
    }
}
