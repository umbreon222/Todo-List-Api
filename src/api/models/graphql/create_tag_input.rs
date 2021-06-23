use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct CreateTagInput {
    pub title: String
}
