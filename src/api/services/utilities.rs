use juniper::{FieldError, FieldResult, graphql_value};

use crate::api::constants::ERROR_DETAILS_KEY;

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(err) => Err(FieldError::from(err))
    }
}

pub fn graphql_error_translate(source: String, error_details: String) -> FieldError {
    FieldError::new(source, graphql_value!({ERROR_DETAILS_KEY: error_details}))
}
