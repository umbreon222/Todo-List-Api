use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct TaskRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub content: String,
    pub priority: i32,
    pub tags: Option<String>,
    pub is_complete: bool,
    pub creation_information_uuid: String
}
