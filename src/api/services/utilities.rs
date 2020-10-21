use juniper::{FieldError, FieldResult};

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(err) => FieldResult::Err(FieldError::from(err)),
    }
}
