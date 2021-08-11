use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[derive(Queryable)]
pub struct UserRow {
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
    pub nickname: String
}
