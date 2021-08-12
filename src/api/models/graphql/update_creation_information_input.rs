use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct UpdateCreationInformationInput {
    pub last_updated_by_user_uuid: String
}
