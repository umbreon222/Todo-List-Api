use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct TagRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub creation_information_uuid: String
}
