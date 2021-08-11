use diesel::sqlite::SqliteConnection;
use juniper::{FieldResult, RootNode};

use crate::api::context::GraphQLContext;
use crate::api::models::graphql::{
    CreateUserInput,
    CreateCreationInformationInput,
    UpdateCreationInformationInput,
    CreateListInput,
    UpdateListInput,
    CreateTaskInput
};
use crate::api::models::database::{UserRow, ListRow, TaskRow};
use crate::api::services::{UserService, ListService};

// The root GraphQL query
pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<UserRow>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a SqliteConnection (for brevity's sake)
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        UserService::all_users(conn)
    }

    #[graphql(name = "allLists")]
    pub fn all_lists(context: &GraphQLContext) -> FieldResult<Vec<ListRow>> {
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        ListService::all_lists(conn)
    }
}

// The root GraphQL mutation
pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    // User
    #[graphql(name = "createUser")]
    pub fn create_user(
        context: &GraphQLContext,
        input: CreateUserInput,
    ) -> FieldResult<UserRow> {
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        UserService::create_user(conn, input)
    }

    // List
    #[graphql(name = "createList")]
    pub fn create_list(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        list_input: CreateListInput
    ) -> FieldResult<ListRow> {
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        ListService::create_list(conn, creation_information_input, list_input )
    }

    #[graphql(name = "updateList")]
    pub fn update_list(
        context: &GraphQLContext,
        update_creation_information_input: UpdateCreationInformationInput,
        update_list_input: UpdateListInput
    ) -> FieldResult<ListRow> {
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        ListService::update_list(conn, update_creation_information_input, update_list_input)
    }

    #[graphql(name = "addNewTask")]
    pub fn update_list(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        create_task_input: CreateTaskInput
    ) -> FieldResult<TaskRow> {
        let conn: &SqliteConnection = &context.pool.get().unwrap();
        ListService::add_new_task(conn, creation_information_input, create_task_input)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
