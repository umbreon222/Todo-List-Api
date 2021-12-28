use crate::api::db::SqlitePool;

pub struct GraphQLContext {
    pub pool: SqlitePool
}

impl juniper::Context for GraphQLContext {}
