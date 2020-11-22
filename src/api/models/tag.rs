use juniper::GraphQLObject;

#[derive(GraphQLObject)]
struct TagRow {
    #[graphql(name = "ID")]
    pub id: i32,
    #[graphql(name = "UUID")]
    pub uuid: String,
    #[graphql(name = "Title")]
    pub title: String,
    #[graphql(name = "CreationInformationUUID")]
    pub creation_information_uuid: i32
}
