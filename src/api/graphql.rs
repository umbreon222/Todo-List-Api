use juniper::{FieldResult, RootNode};


use crate::api::models::database::{
    UserRow,
    ListRow,
    TaskRow,
    CreationInformationRow
};
use crate::api::services::{
    UserService,
    ListService,
    TaskService,
    CreationInformationService
};
use crate::api::models::graphql::{
    CreateUserInput,
    CreateCreationInformationInput,
    UpdateCreationInformationInput,
    CreateListInput,
    UpdateListInput,
    CreateTaskInput,
    AddTaskInput
};
use crate::api::context::GraphQLContext;

// The root GraphQL query
pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allUsers")]
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<UserRow>> {
        // TODO: pass the GraphQLContext into the querying functions rather
        // than a SqliteConnection (for brevity's sake)
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        user_service.all_users()
    }

    #[graphql(name = "allLists")]
    pub fn all_lists(context: &GraphQLContext) -> FieldResult<Vec<ListRow>> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        let list_service = ListService::new(&connection, &creation_information_service, &task_service);
        list_service.all_lists()
    }

    #[graphql(name = "allTasks")]
    pub fn all_tasks(context: &GraphQLContext) -> FieldResult<Vec<TaskRow>> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        task_service.all_tasks()
    }

    #[graphql(name = "allCreationInformation")]
    pub fn all_creation_information(context: &GraphQLContext) -> FieldResult<Vec<CreationInformationRow>> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        creation_information_service.all_creation_information()
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
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        user_service.create_user(input)
    }

    // List
    #[graphql(name = "createList")]
    pub fn create_list(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        list_input: CreateListInput
    ) -> FieldResult<ListRow> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        let list_service = ListService::new(&connection, &creation_information_service, &task_service);
        list_service.create_list(creation_information_input, list_input)
    }

    #[graphql(name = "updateList")]
    pub fn update_list(
        context: &GraphQLContext,
        update_creation_information_input: UpdateCreationInformationInput,
        update_list_input: UpdateListInput
    ) -> FieldResult<ListRow> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        let list_service = ListService::new(&connection, &creation_information_service, &task_service);
        list_service.update_list(update_creation_information_input, update_list_input)
    }

    #[graphql(name = "addTask")]
    pub fn add_task(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        add_task_input: AddTaskInput
    ) -> FieldResult<ListRow> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        let list_service = ListService::new(&connection, &creation_information_service, &task_service);
        list_service.add_task(add_task_input)
    }

    #[graphql(name = "createTask")]
    pub fn create_task(
        context: &GraphQLContext,
        creation_information_input: CreateCreationInformationInput,
        create_task_input: CreateTaskInput
    ) -> FieldResult<TaskRow> {
        let connection = context.pool.get().unwrap();
        let user_service = UserService::new(&connection);
        let creation_information_service = CreationInformationService::new(&connection, &user_service);
        let task_service = TaskService::new(&connection, &creation_information_service);
        task_service.create_task(creation_information_input, create_task_input)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
