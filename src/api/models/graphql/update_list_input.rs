use juniper::GraphQLInputObject;

use crate::api::models::List;
use crate::api::models::utilities::parse_color_hex;

#[derive(GraphQLInputObject)]
pub struct UpdateListInput {
    pub uuid: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub color_hex: Option<String>,
}

impl UpdateListInput {
    pub fn create_updated_list(&self, list: List) -> Result<List, String> {
        let title = self.title.clone().unwrap_or(list.title);
        // Update description
        let description: Option<String>;
        match &self.description {
            Some(res) => {
                description = Some(res.to_string());
            }
            None => {
                description = list.description;
            }
        }
        // Update color hex
        let color_hex: Option<String>;
        match &self.color_hex {
            Some(string_color_hex) => {
                match parse_color_hex(&string_color_hex) {
                    Ok(res) => {
                        color_hex = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                color_hex = list.color_hex;
            }
        }
        Ok(List {
            uuid: list.uuid.clone(),
            title,
            description,
            color_hex,
            task_uuids: list.task_uuids.clone(),
            parent_list_uuid: list.parent_list_uuid.clone(),
            sub_list_uuids: list.sub_list_uuids.clone(),
            shared_with_user_uuids: list.shared_with_user_uuids.clone(),
            creation_information_uuid: list.creation_information_uuid.clone()
        })
    }
}
