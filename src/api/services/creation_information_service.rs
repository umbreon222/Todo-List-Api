use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants::{ERROR_DETAILS_KEY, CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE, INTERNAL_ERROR};
use crate::api::{models, schema};
use schema::CreationInformation::dsl::*;
use crate::api::services::UserService;
use crate::api::services::utilities::graphql_translate;

pub struct CreationInformationService;

impl CreationInformationService {
    pub fn all_creation_information(conn: &SqliteConnection) -> FieldResult<Vec<models::CreationInformationRow>> {
        graphql_translate(CreationInformation.load::<models::CreationInformationRow>(conn))
    }

    pub fn create_creation_information(
        conn: &SqliteConnection,
        create_creation_information_input: models::CreateCreationInformationInput
    ) -> FieldResult<models::CreationInformationRow> {
        // Parse create creation information input
        let new_creation_information: models::CreationInformationStruct;
        match create_creation_information_input.create_creation_information() {
            Ok(res) => {
                new_creation_information = res;
            },
            Err(err) => {
                return Err(FieldError::new(CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
            }
        }
        // Verify the given creator user uuid exists
        match UserService::user_exists(conn, &new_creation_information.creator_user_uuid.to_string()) {
            Ok(user_exists) => {
                if !user_exists {
                    let err_details = format!("The user '{}' does not exist", new_creation_information.creator_user_uuid.to_string());
                    return Err(FieldError::new(CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_details })));
                }
            },
            Err(err) => {
                let error_details = err.message();
                return Err(FieldError::new(CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
            }
        }
        // Create new creation information row
        let new_creation_information_row = new_creation_information.create_new_creation_information_row();
        // Execute insertion
        match diesel::insert_into(schema::CreationInformation::table).values(&new_creation_information_row).execute(conn) {
            Ok(_) => {},
            Err(err) => {
                let error_details = err.to_string();
                return Err(FieldError::new(CREATION_INFORMATION_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
            }
        }
        // Return error or newly inserted row via UUID look up
        match CreationInformationService::get_creation_information_by_uuid(&conn, &new_creation_information.uuid.to_string()) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!("Couldn't find creation information '{}' after insert", new_creation_information.uuid.to_string());
                        Err(FieldError::new(INTERNAL_ERROR, graphql_value!({ ERROR_DETAILS_KEY: error_details })))
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_creation_information_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::CreationInformationRow>> {
        match CreationInformation.filter(UUID.eq(uuid.clone())).first::<models::CreationInformationRow>(conn) {
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
        
        return graphql_translate(select(exists(CreationInformation.filter(UUID.eq(uuid.clone())))).get_result::<bool>(conn));
    }
}
