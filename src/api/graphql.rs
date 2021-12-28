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
use crate::api::models::database::{
    UserRow,
    ListRow,
    TaskRow,
    CreationInformationRow
};

// The root GraphQL query
pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<UserRow>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a SqliteConnection (for brevity's sake)
        context.user_service.all_users()
    }

    #[graphql(name = "allLists")]
    pub fn all_lists(context: &GraphQLContext) -> FieldResult<Vec<ListRow>> {
        context.list_service.all_lists()
    }

    #[graphql(name = "allTasks")]
    pub fn all_lists(context: &GraphQLContext) -> FieldResult<Vec<TaskRow>> {
        context.task_service.all_tasks()
    }

    #[graphql(name = "allCreationInformation")]
    pub fn all_lists(context: &GraphQLContext) -> FieldResult<Vec<CreationInformationRow>> {
        context.creation_information_service.all_creation_information()
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
        context.user_service.create_user(input)
    }

    // List
    #[graphql(name = "createList")]
    pub fn create_list(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        list_input: CreateListInput
    ) -> FieldResult<ListRow> {
        context.list_service.create_list(&context.creation_information_service, &context.user_service, creation_information_input, list_input)
    }

    #[graphql(name = "updateList")]
    pub fn update_list(
        context: &GraphQLContext,
        update_creation_information_input: UpdateCreationInformationInput,
        update_list_input: UpdateListInput
    ) -> FieldResult<ListRow> {
        context.list_service.update_list(&context.creation_information_service, &context.user_service, update_creation_information_input, update_list_input)
    }

    #[graphql(name = "addNewTask")]
    pub fn update_list(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        create_task_input: CreateTaskInput
    ) -> FieldResult<TaskRow> {
        context.list_service.add_new_task(&context.creation_information_service, &context.user_service, &context.task_service, creation_information_input, create_task_input)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
