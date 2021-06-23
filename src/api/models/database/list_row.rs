use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct ListRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<String>,
    pub parent_list_uuid: Option<String>,
    pub sub_list_uuids: Option<String>,
    pub shared_with_user_uuids: Option<String>,
    pub creation_information_uuid: String
}
