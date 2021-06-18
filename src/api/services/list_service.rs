use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::lists::dsl;
use crate::api::services::{CreationInformationService, TaskService, UserService};
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct ListService;

impl ListService {
    pub fn all_lists(conn: &SqliteConnection) -> FieldResult<Vec<models::ListRow>> {
        graphql_translate(dsl::lists.load::<models::ListRow>(conn))
    }

    pub fn create_list(
        conn: &SqliteConnection,
        new_creation_information_input: models::CreateCreationInformationInput,
        create_list_input: models::CreateListInput
    ) -> FieldResult<models::ListRow> {
        // Create creation information
        let creation_information: models::CreationInformationStruct;
        match CreationInformationService::create_creation_information(
            conn,
            new_creation_information_input
        ) {
            Ok(creation_information_row) => {
                match creation_information_row.create_creation_information_struct() {
                    Ok(res) => {
                        creation_information = res;
                    },
                    Err(err) => {
                        return Err(graphql_error_translate(
                            constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                            err
                        ));
                    }
                }
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Parse create list input
        let new_list: models::List;
        match create_list_input.create_list(&creation_information.uuid) {
            Ok(list) => {
                new_list = list;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err
                ));
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
                                let error_details: String = format!(
                                    "The task '{}' does not exist",
                                    task_uuid.to_string()
                                );
                                return Err(graphql_error_translate(
                                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                    error_details
                                ));
                            }
                        },
                        Err(err) => {
                            return Err(graphql_error_translate(
                                constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                err.message().to_string()
                            ));
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
                            let err_details = format!(
                                "The parent list '{}' does not exist",
                                list_uuid.to_string()
                            );
                            return Err(graphql_error_translate(
                                constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                err_details
                            ));
                        }
                    },
                    Err(err) => {
                        return Err(graphql_error_translate(
                            constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                            err.message().to_string()
                        ));
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
                                let err_details = format!(
                                    "The sub list '{}' does not exist",
                                    sub_list_uuid.to_string()
                                );
                                return Err(graphql_error_translate(
                                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                    err_details
                                ));
                            }
                        },
                        Err(err) => {
                            return Err(graphql_error_translate(
                                constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                err.message().to_string()
                            ));
                        }
                    }
                }
            },
            None => {}
        }
        // Verify the shared with user uuids exist
        match &new_list.shared_with_user_uuids {
            Some(shared_with_user_uuids) => {
                for shared_with_user_uuid in shared_with_user_uuids {
                    match UserService::user_exists(conn, &shared_with_user_uuid.to_string()) {
                        Ok(user_exists) => {
                            if !user_exists {
                                let err_details = format!(
                                    "The user '{}' does not exist",
                                    shared_with_user_uuid.to_string()
                                );
                                return Err(graphql_error_translate(
                                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                    err_details
                                ));
                            }
                        },
                        Err(err) => {
                            return Err(graphql_error_translate(
                                constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                                err.message().to_string()
                            ));
                        }
                    }
                }
            },
            None => {}
        }
        // Create new list row
        let new_list_row: models::NewListRow;
        match new_list.create_new_list_row() {
            Ok(res) => {
                new_list_row = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Execute insertion
        match diesel::insert_into(schema::lists::table)
            .values(&new_list_row)
            .execute(conn) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::LIST_NOT_CREATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Return error or newly inserted row via UUID look up
        match ListService::get_list_by_uuid(&conn, &new_list.uuid.to_string()) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find list '{}' after insert",
                            new_list.uuid.to_string()
                        );
                        Err(graphql_error_translate(
                            constants::INTERNAL_ERROR.to_string(),
                            error_details
                        ))
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_list_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::ListRow>> {
        match dsl::lists.filter(dsl::uuid.eq(uuid.clone())).first::<models::ListRow>(conn) {
            Ok(list_row) => Ok(Some(list_row)),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(FieldError::from(err)),
            }
        }
    }

    pub fn list_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(
            select(exists(dsl::lists.filter(dsl::uuid.eq(uuid.clone()))))
                .get_result::<bool>(conn)
        );
    }

    pub fn add_task(
        conn: &SqliteConnection,
        list_uuid: &String,
        task_uuid: &String
    ) -> FieldResult<models::ListRow> {
        // Grab id and current task uuids from the list
        let list_id: i32;
        let task_uuids: Option<String>;
        let query = dsl::lists.select((dsl::id, dsl::task_uuids))
            .filter(dsl::uuid.eq(list_uuid.clone()))
            .first::<(i32, Option<String>)>(conn);
        match query {
            Ok(res) => {
                list_id = res.0;
                task_uuids = res.1;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                    err.to_string()
                ));
            }
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
                        return Err(graphql_error_translate(
                            constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                            err.to_string()
                        ));
                    },
                }
            },
            None => {}
        }
        // Add the task to the list
        updated_task_uuids.push(task_uuid.clone());
        // Convert the modified task uuids list back to a json string
        let updated_task_uuids_json = serde_json::to_string(&updated_task_uuids)?;
        // Create update list row
        let updated = diesel::update(dsl::lists.find(list_id))
            .set(dsl::task_uuids.eq(updated_task_uuids_json))
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match updated {
            Ok(_size) => {
                graphql_translate(
                    dsl::lists
                        .filter(dsl::uuid.eq(list_uuid.clone()))
                        .first::<models::ListRow>(conn)
                )
            },
            Err(err) => {
                Err(graphql_error_translate(
                    constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                    err.to_string()
                ))
            }
        }
    }
}
