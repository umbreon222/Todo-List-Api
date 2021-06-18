use std::sync::Arc;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use actix_web::{web, Error, HttpResponse};

use crate::api::db::SqlitePool;
use crate::api::graphql::{create_schema, Schema};
use crate::api::context::GraphQLContext;

// GraphQL endpoint configuration callback
pub fn graphql_endpoints(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .data(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphql", web::get().to(graphql_playground));
}

// GraphQL playground route.
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/graphql"))
}

// Core GraphQL handler that provides all functionality.
async fn graphql(
    // The DB connection pool
    pool: web::Data<SqlitePool>,
    // The GraphQL schema
    schema: web::Data<Arc<Schema>>,
    // The incoming HTTP request
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // Instantiate a context
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned()
    };

    // Handle the incoming request and return a string result (or error)
    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    // Return the string as a JSON payload
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
