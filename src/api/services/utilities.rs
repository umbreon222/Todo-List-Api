use diesel::sqlite::SqliteConnection;
use juniper::{FieldError, FieldResult};

use crate::api::errors::ValidationError;

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(err) => FieldResult::Err(FieldError::from(err)),
    }
}

pub fn verify_json_uuids_exist_and_parse(
    json_uuids: &String,
    conn: &SqliteConnection,
    uuid_exists: impl Fn(&SqliteConnection, String) -> FieldResult<bool>
) -> Result<Vec<String>, ValidationError> {
    // Deserialize the task uuids
    let uuids: Vec<String>;
    match serde_json::from_str::<Vec<String>>(&json_uuids) {
        Ok(parsed_uuids) => {
            uuids = parsed_uuids;
        },
        Err(err) => {
            let err_string = err.to_string();
            return Err(ValidationError::new(&err_string));
        },
    }
    for uuid in &uuids {
        // Verify that the task uuids exist
        match uuid_exists(&conn, uuid.to_string()) {
            Ok(uuid_exists) => {
                if !uuid_exists {
                    let error_details: String = format!("The uuid '{}' does not exist", uuid.to_string());
                    return Err(ValidationError::new(&error_details));
                }
            },
            Err(err) => return Err(ValidationError::new(&err.message())),
        }
    }
    Ok(uuids)
}
