use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct UserRow {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}
