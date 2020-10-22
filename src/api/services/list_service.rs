use uuid::Uuid;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants;
use crate::api::{schema, models};
use crate::api::services::{CreationInformationService, UserService, TaskService};
use crate::api::services::utilities::{graphql_translate, verify_json_uuids_exist_and_parse};

pub struct ListService;

impl ListService {
    pub fn all_lists(conn: &SqliteConnection) -> FieldResult<Vec<models::List>> {
        use schema::Lists::dsl::*;
        
        graphql_translate(Lists.load::<models::List>(conn))
    }

    pub fn create_list(
        conn: &SqliteConnection,
        new_creation_information: models::CreateCreationInformationInput,
        new_list: models::CreateListInput
    ) -> FieldResult<models::List> {
        use schema::Lists::dsl::*;

        // Validate input 
        match new_list.validate() {
            Ok(_) => {},
            Err(validation_error) => {
                let err_details_key = constants::ERROR_DETAILS_KEY;
                let err_details = validation_error.to_string();
                return FieldResult::Err(FieldError::new("Invalid input for new list", graphql_value!({ err_details_key: err_details })));
            },
        }
        // Verify the given task uuids exist
        match &new_list.task_uuids {
            Some(json_task_uuids) => {
                match verify_json_uuids_exist_and_parse(json_task_uuids, &conn, TaskService::task_exists) {
                    Ok(_) => {},
                    Err(validation_error) => {
                        let err_details_key = constants::ERROR_DETAILS_KEY;
                        let err_details = validation_error.to_string();
                        return FieldResult::Err(FieldError::new("Task UUID not found", graphql_value!({ err_details_key: err_details })));
                    },
                }
            },
            None => {},
        }
        // Verify the parent list uuid exists
        match &new_list.parent_list_uuid {
            Some(list_uuid) => {
                match ListService::list_exists(&conn, list_uuid.to_string()) {
                    Ok(list_exists) => {
                        if !list_exists {
                            let err_details_key = constants::ERROR_DETAILS_KEY;
                            let err_details = format!("The uuid '{}' does not exist", list_uuid.to_string());
                            return FieldResult::Err(FieldError::new("Parent list UUID not found", graphql_value!({ err_details_key: err_details })));
                        }
                    },
                    Err(err) => return Err(err),
                }
            },
            None => {},
        }
        // Verify the sub list uuids exist
        match &new_list.sub_list_uuids {
            Some(json_sub_list_uuids) => {
                match verify_json_uuids_exist_and_parse(json_sub_list_uuids, &conn, ListService::list_exists) {
                    Ok(_) => {},
                    Err(validation_error) => {
                        let err_details_key = constants::ERROR_DETAILS_KEY;
                        let err_details = validation_error.to_string();
                        return FieldResult::Err(FieldError::new("Sub-list UUID not found", graphql_value!({ err_details_key: err_details })));
                    },
                }
            },
            None => {},
        }
        // Verify the creator user uuid exists
        match UserService::user_exists(&conn, new_creation_information.creator_user_uuid.to_string()) {
            Ok(user_exists) => {
                if !user_exists {
                    let err_details_key = constants::ERROR_DETAILS_KEY;
                    let err_details = format!("The uuid '{}' does not exist", new_creation_information.creator_user_uuid.to_string());
                    return FieldResult::Err(FieldError::new("Creator UUID not found", graphql_value!({ err_details_key: err_details })));
                }
            },
            Err(err) => return Err(err),
        }
        // By default, the list being created will be shared with the creator.
        let mut updated_shared_with_user_uuids: Vec<String> = vec![new_creation_information.creator_user_uuid.to_string()];
        // Verify and append any additional users uuids that the creator wants to share with if need be
        match new_list.shared_with_user_uuids {
            Some(json_user_uuids) => {
                let mut user_uuids: Vec<String>;
                match verify_json_uuids_exist_and_parse(&json_user_uuids, &conn, UserService::user_exists) {
                    Ok(parsed_user_uuids) => {
                        user_uuids = parsed_user_uuids;
                    },
                    Err(validation_error) => {
                        let err_details_key = constants::ERROR_DETAILS_KEY;
                        let err_details = validation_error.to_string();
                        return FieldResult::Err(FieldError::new("User UUID not found", graphql_value!({ err_details_key: err_details })));
                    },
                }
                updated_shared_with_user_uuids.append(&mut user_uuids);
            },
            None => {},
        }
        // Convert the modified shared user ids list back to a json string
        let updated_shared_with_user_uuids_json = serde_json::to_string(&updated_shared_with_user_uuids)?;
        // Use creation information service to create a creation information object in db
        let created_creation_information: models::CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, new_creation_information) {
            Ok(created) => {
                created_creation_information = created;
            },
            Err(err) => return Err(err),
        };
        // Create new list row
        let uuid = Uuid::new_v4();
        let new_list = models::NewList {
            UUID: &uuid.to_string(),
            Title: &new_list.title,
            Description: new_list.description,
            ColorHex: new_list.color_hex,
            TaskUUIDs: new_list.task_uuids,
            ParentListUUID: new_list.parent_list_uuid,
            SubListUUIDs: new_list.sub_list_uuids,
            SharedWithUserUUIDs: Some(updated_shared_with_user_uuids_json),
            CreationInformationUUID: &created_creation_information.uuid
        };
        // Execute insertion
        let inserted = diesel::insert_into(schema::Lists::table)
            .values(&new_list)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Lists.filter(UUID.eq(uuid.to_string())).first::<models::List>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("List not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_list_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<models::List>> {
        use schema::Lists::dsl::*;

        match Lists.filter(UUID.eq(uuid)).first::<models::List>(conn) {
            Ok(list) => Ok(Some(list)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }

    pub fn list_exists(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<bool> {
        use schema::Lists::dsl::*;
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Lists.filter(UUID.eq(uuid)))).get_result::<bool>(conn));
    }

    pub fn add_task(
        conn: &SqliteConnection,
        list_uuid: String,
        task_uuid: String
    ) -> FieldResult<models::List> {
        use schema::Lists::dsl::*;

        // Grab id and current task uuids from the list
        let list_id: i32;
        let task_uuids: Option<String>;
        let query = Lists.select((ID, TaskUUIDs))
            .filter(UUID.eq(list_uuid.clone()))
            .first::<(i32, Option<String>)>(conn);
        match query {
            Ok(res) => {
                list_id = res.0;
                task_uuids = res.1;
            },
            Err(err) => {
                let err_string = err.to_string();
                return FieldResult::Err(FieldError::new("Task not added", graphql_value!({ "internal_error": err_string })));
            },
        };
        // Convert task uuids json to vector
        let mut updated_task_uuids: Vec<String> = vec![];
        match task_uuids {
            Some(task_uuids_json) => {
                match serde_json::from_str(&task_uuids_json) {
                    Ok(mut task_uuids) => {
                        updated_task_uuids.append(&mut task_uuids);
                    },
                    Err(err) => {
                        let err_string = err.to_string();
                        return FieldResult::Err(FieldError::new("Task not added", graphql_value!({ "internal_error": err_string })));
                    },
                }
            },
            None => {},
        }
        // Add the task to the list
        updated_task_uuids.push(task_uuid);
        // Convert the modified task uuids list back to a json string
        let updated_task_uuids_json = serde_json::to_string(&updated_task_uuids)?;
        // Create update list row
        let updated = diesel::update(Lists.find(list_id))
            .set(TaskUUIDs.eq(updated_task_uuids_json))
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match updated {
            Ok(_size) => {
                graphql_translate(Lists.filter(UUID.eq(list_uuid)).first::<models::List>(conn))
            },
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Task not added", graphql_value!({ "internal_error": err_string })))
            },
        }
    }
}
