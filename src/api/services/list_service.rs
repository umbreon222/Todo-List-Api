use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants::{ERROR_DETAILS_KEY, LIST_NOT_CREATED_ERROR_MESSAGE, TASK_NOT_ADDED_ERROR_MESSAGE};
use crate::api::{models, schema};
use schema::Lists::dsl::*;
use crate::api::services::{CreationInformationService, TaskService};
use crate::api::services::utilities::graphql_translate;

pub struct ListService;

impl ListService {
    pub fn all_lists(conn: &SqliteConnection) -> FieldResult<Vec<models::ListRow>> {
        graphql_translate(Lists.load::<models::ListRow>(conn))
    }

    pub fn create_list(
        conn: &SqliteConnection,
        new_creation_information_input: models::CreateCreationInformationInput,
        create_list_input: models::CreateListInput
    ) -> FieldResult<models::ListRow> {
        // Create creation information
        let creation_information: models::CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, new_creation_information_input) {
            Ok(creation_information_row) => {
                match creation_information_row.create_creation_information_struct() {
                    Ok(res) => {
                        creation_information = res;
                    },
                    Err(err) => {
                        return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
                    }
                }
            },
            Err(err) => {
                let err_details = err.message();
                return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_details })));
            }
        }
        // Create List
        let new_list: models::List;
        match create_list_input.create_list(&creation_information.uuid) {
            Ok(list) => {
                new_list = list;
            },
            Err(err) => {
                return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
            }
        }
        // Verify the given task uuids exist
        match &new_list.task_uuids {
            Some(task_uuids) => {
                for task_uuid in task_uuids {
                    // Verify that the task uuids exist
                    match TaskService::task_exists(conn, &task_uuid.to_string()) {
                        Ok(task_exists) => {
                            if !task_exists {
                                let error_details: String = format!("Task '{}' does not exist", task_uuid.to_string());
                                return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
                            }
                        },
                        Err(err) => {
                            let error_details = err.message();
                            return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
                        }
                    }
                }
            },
            None => {}
        }
        // Verify the parent list uuid exists
        match &new_list.parent_list_uuid {
            Some(list_uuid) => {
                match ListService::list_exists(conn, &list_uuid.to_string()) {
                    Ok(list_exists) => {
                        if !list_exists {
                            let err_details = format!("The parent list '{}' does not exist", list_uuid.to_string());
                            return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_details })));
                        }
                    },
                    Err(err) => {
                        let error_details = err.message();
                        return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
                    }
                }
            },
            None => {}
        }
        // Verify the sub list uuids exist
        match &new_list.sub_list_uuids {
            Some(sub_list_uuids) => {
                for sub_list_uuid in sub_list_uuids {
                    match ListService::list_exists(conn, &sub_list_uuid.to_string()) {
                        Ok(list_exists) => {
                            if !list_exists {
                                let err_details = format!("The sub list '{}' does not exist", sub_list_uuid.to_string());
                                return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_details })));
                            }
                        },
                        Err(err) => {
                            let error_details = err.message();
                            return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: error_details })));
                        }
                    }
                }
            },
            None => {},
        }
        let new_list_row: models::NewListRow;
        match new_list.create_new_list_row() {
            Ok(res) => {
                new_list_row = res;
            },
            Err(err) => {
                return FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
            }
        }
        // Execute insertion
        let inserted = diesel::insert_into(schema::Lists::table)
            .values(&new_list_row)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Lists.filter(UUID.eq(new_list.uuid.to_string())).first::<models::ListRow>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new(LIST_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })))
            },
        }
    }

    pub fn get_list_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::ListRow>> {
        match Lists.filter(UUID.eq(uuid.clone())).first::<models::ListRow>(conn) {
            Ok(list_row) => Ok(Some(list_row)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }

    pub fn list_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Lists.filter(UUID.eq(uuid.clone())))).get_result::<bool>(conn));
    }

    pub fn add_task(
        conn: &SqliteConnection,
        list_uuid: &String,
        task_uuid: &String
    ) -> FieldResult<models::ListRow> {
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
                return FieldResult::Err(FieldError::new(TASK_NOT_ADDED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })));
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
                        return FieldResult::Err(FieldError::new(TASK_NOT_ADDED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })));
                    },
                }
            },
            None => {},
        }
        // Add the task to the list
        updated_task_uuids.push(task_uuid.clone());
        // Convert the modified task uuids list back to a json string
        let updated_task_uuids_json = serde_json::to_string(&updated_task_uuids)?;
        // Create update list row
        let updated = diesel::update(Lists.find(list_id))
            .set(TaskUUIDs.eq(updated_task_uuids_json))
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match updated {
            Ok(_size) => {
                graphql_translate(Lists.filter(UUID.eq(list_uuid.clone())).first::<models::ListRow>(conn))
            },
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new(TASK_NOT_ADDED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })))
            },
        }
    }
}
