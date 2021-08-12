use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct UpdateListInput {
    pub uuid: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color_hex: Option<String>,
}
