use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::creation_information::dsl;
use crate::api::services::UserService;
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct CreationInformationService;

impl CreationInformationService {
    pub fn all_creation_information(
        conn: &SqliteConnection
    ) -> FieldResult<Vec<models::database::CreationInformationRow>> {
        graphql_translate(dsl::creation_information.load::<models::database::CreationInformationRow>(conn))
    }

    pub fn create_creation_information(
        conn: &SqliteConnection,
        create_creation_information_input: models::graphql::CreateCreationInformationInput
    ) -> FieldResult<models::database::CreationInformationRow> {
        // Parse create creation information input
        let new_creation_information: models::CreationInformation;
        match create_creation_information_input.create_creation_information() {
            Ok(res) => {
                new_creation_information = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::INTERNAL_ERROR.to_string(),
                    err
                ));
            }
        }
        // Verify the given creator user uuid exists
        match UserService::user_exists(
            conn,
            &new_creation_information.creator_user_uuid.to_string()
        ) {
            Ok(user_exists) => {
                if !user_exists {
                    let err_details = format!(
                        "The user '{}' does not exist",
                        new_creation_information.creator_user_uuid.to_string()
                    );
                    return Err(graphql_error_translate(
                        constants::INTERNAL_ERROR.to_string(),
                        err_details
                    ));
                }
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::INTERNAL_ERROR.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Create new creation information row
        let new_creation_information_row = new_creation_information
            .create_new_creation_information_row();
        // Execute insertion
        match diesel::insert_into(schema::creation_information::table)
            .values(&new_creation_information_row)
            .execute(conn) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Return error or newly inserted row via UUID look up
        match CreationInformationService::get_creation_information_by_uuid(
            &conn,
            &new_creation_information.uuid.to_string()
        ) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find creation information '{}' after insert",
                            new_creation_information.uuid.to_string()
                        );
                        Err(graphql_error_translate(
                            constants::INTERNAL_ERROR.to_string(),
                            error_details
                        ))
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_creation_information_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::database::CreationInformationRow>> {
        match dsl::creation_information
            .filter(dsl::uuid.eq(uuid))
            .first::<models::database::CreationInformationRow>(conn) {
                Ok(creation_information) => Ok(Some(creation_information)),
                Err(err) => match err {
                    diesel::result::Error::NotFound => Ok(None),
                    _ => Err(FieldError::from(err))
                }
            }
    }

    pub fn creation_information_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        graphql_translate(
            select(
                exists(dsl::creation_information.filter(dsl::uuid.eq(uuid)))
            ).get_result::<bool>(conn)
        )
    }

    pub fn update_creation_information (
        conn: &SqliteConnection,
        uuid: &String,
        update_creation_information_input: models::graphql::UpdateCreationInformationInput
    ) -> FieldResult<models::database::CreationInformationRow> {
        // Find the creation information row to update
        let creation_information_row: models::database::CreationInformationRow;
        match CreationInformationService::get_creation_information_by_uuid(
            &conn,
            &uuid
        ) {
            Ok(res) => {
                match res {
                    Some(found_creation_information_row) => {
                        creation_information_row = found_creation_information_row;
                    },
                    None => {
                        return Err(
                            graphql_error_translate(
                                constants::CREATION_INFORMATION_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                                format!("Creation information '{}' not found", uuid)
                            )
                        );
                    }
                }
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::CREATION_INFORMATION_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Create creation information from creation information row
        let creation_information: models::CreationInformation;
        match creation_information_row.create_creation_information() {
            Ok(res) => {
                creation_information = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::CREATION_INFORMATION_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Update creation information
        let updated_creation_information: models::CreationInformation;
        match update_creation_information_input.create_updated_creation_information(creation_information) {
            Ok(res) => {
                updated_creation_information = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::CREATION_INFORMATION_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Convert updated creation information back to creation information row
        let updated_creation_information_row = updated_creation_information.create_updated_creation_information_row(creation_information_row);
        // Execute Update
        match diesel::update(dsl::creation_information.filter(dsl::uuid.eq(updated_creation_information_row.uuid.clone())))
            .set((
                dsl::last_updated_by_user_uuid.eq(updated_creation_information_row.last_updated_by_user_uuid.clone()),
                dsl::last_updated_time.eq(updated_creation_information_row.last_updated_time.clone())
            )).execute(conn) {
                Ok(_) => Ok(updated_creation_information_row),
                Err(err) => {
                    Err(graphql_error_translate(
                        constants::CREATION_INFORMATION_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ))
                }
            }
    }
}
