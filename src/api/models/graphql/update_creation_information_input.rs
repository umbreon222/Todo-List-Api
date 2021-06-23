use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct UpdateCreationInformationInput {
    pub last_updated_by_user_uuid: String
}

/*impl UpdateCreationInformationInput {
    pub fn create_update_creation_information(&self) -> Result<CreationInformationStruct, String> {
        // Generate uuid for new creation information
        let uuid = Uuid::new_v4();
        // Parse creator user uuid
        match Uuid::parse_str(&self.creator_user_uuid) {
            Ok(creator_user_uuid) => {
                let current_time = Utc::now();
                Ok(CreationInformationStruct {
                    uuid,
                    creator_user_uuid: creator_user_uuid.clone(),
                    creation_time: current_time.clone(),
                    last_updated_by_user_uuid: creator_user_uuid,
                    last_updated_time: current_time
                })
            },
            Err(err) => Err(err.to_string())
        }
    }
}*/
