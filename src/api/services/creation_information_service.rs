use chrono::prelude::*;
use uuid::Uuid;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::services::utilities::graphql_translate;
use crate::api::schema;
use crate::api::models;

pub struct CreationInformationService;

impl CreationInformationService {
    pub fn all_creation_information(conn: &SqliteConnection) -> FieldResult<Vec<models::CreationInformationStruct>> {
        use schema::CreationInformation::dsl::*;

        graphql_translate(CreationInformation.load::<models::CreationInformationStruct>(conn))
    }

    pub fn create_creation_information(
        conn: &SqliteConnection,
        new_creation_information: models::CreateCreationInformationInput
    ) -> FieldResult<models::CreationInformationStruct> {
        use schema::CreationInformation::dsl::*;

        // Create new creation information row
        let uuid = Uuid::new_v4();
        let current_time_string = Utc::now().to_rfc3339();
        let new_creation_information = models::NewCreationInformation {
            UUID: &uuid.to_string(),
            CreatorUserUUID: &new_creation_information.creator_user_uuid,
            CreationTime: &current_time_string,
            LastUpdatedByUserUUID: &new_creation_information.creator_user_uuid,
            LastUpdatedTime: &current_time_string
        };
        // Execute insertion
        let inserted = diesel::insert_into(schema::CreationInformation::table)
            .values(&new_creation_information)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(CreationInformation.filter(UUID.eq(uuid.to_string())).first::<models::CreationInformationStruct>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Creation information not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_creation_information_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<models::CreationInformationStruct>> {
        use schema::CreationInformation::dsl::*;

        match CreationInformation.filter(UUID.eq(uuid)).first::<models::CreationInformationStruct>(conn) {
            Ok(creation_information) => Ok(Some(creation_information)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }

    pub fn creation_information_exists(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<bool> {
        use schema::CreationInformation::dsl::*;
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(CreationInformation.filter(UUID.eq(uuid)))).get_result::<bool>(conn));
    }
}
