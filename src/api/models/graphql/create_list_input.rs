use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
}
