use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct AddTaskInput {
    pub last_updated_by_user_uuid: String,
    pub parent_list_uuid: String,
    pub task_uuid: String
}
