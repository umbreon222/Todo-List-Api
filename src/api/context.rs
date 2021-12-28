use super::db::SqlitePool;
use crate::api::services::{
    CreationInformationService,
    UserService,
    TaskService,
    ListService
};

pub struct GraphQLContext {
    pub pool: SqlitePool,
    pub creation_information_service: CreationInformationService<'static>,
    pub user_service: UserService<'static>,
    pub task_service: TaskService<'static>,
    pub list_service: ListService<'static>
}

impl juniper::Context for GraphQLContext {}
