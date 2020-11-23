use juniper::{FieldError, FieldResult, graphql_value};

use crate::api::constants;

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(err) => Err(FieldError::from(err))
    }
}

pub fn error_translate<T>(source: String, error_details: String) -> FieldResult<T> {
    let err_details_key = constants::ERROR_DETAILS_KEY;
    Err(FieldError::new(source, graphql_value!({ err_details_key: error_details })))
}
