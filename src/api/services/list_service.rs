use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::lists::dsl;
use crate::api::services::{CreationInformationService, TaskService};
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct ListService;

impl ListService {
    pub fn all_lists(conn: &SqliteConnection) -> FieldResult<Vec<models::database::ListRow>> {
        graphql_translate(dsl::lists.load::<models::database::ListRow>(conn))
    }

    pub fn create_list(
        conn: &SqliteConnection,
        new_creation_information_input: models::graphql::CreateCreationInformationInput,
        create_list_input: models::graphql::CreateListInput
    ) -> FieldResult<models::database::ListRow> {
        // Create creation information
        let creation_information: models::CreationInformation;
        match CreationInformationService::create_creation_information(
            conn,
            new_creation_information_input
        ) {
            Ok(creation_information_row) => {
                match models::CreationInformation::from_creation_information_row(creation_information_row) {
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
        match models::List::from_create_list_input(create_list_input, creation_information.uuid) {
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
        // TODO: Break this up into smaller functions
        /*
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
        */
        // Create new list row
        let new_list_row = models::database::NewListRow::from_list(new_list);
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
        match ListService::get_list_by_uuid(&conn, &new_list_row.uuid) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find list '{}' after insert",
                            &new_list_row.uuid
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
    ) -> FieldResult<Option<models::database::ListRow>> {
        match dsl::lists.filter(dsl::uuid.eq(uuid)).first::<models::database::ListRow>(conn) {
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
            select(exists(dsl::lists.filter(dsl::uuid.eq(uuid))))
                .get_result::<bool>(conn)
        );
    }

    pub fn update_list(
        conn: &SqliteConnection,
        update_creation_information_input: models::graphql::UpdateCreationInformationInput,
        update_list_input: models::graphql::UpdateListInput
    ) -> FieldResult<models::database::ListRow> {
        // Find the list row to update
        let list_row: models::database::ListRow;
        match ListService::get_list_by_uuid(&conn, &update_list_input.uuid) {
            Ok(result) => {
                match result {
                    Some(found_list_row) => {
                        list_row = found_list_row;
                    },
                    None => {
                        return Err(
                            graphql_error_translate(
                                constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                                format!("List '{}' not found", update_list_input.uuid.to_string())
                            )
                        );
                    }
                }
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Create list from list row
        let mut list: models::List;
        match models::List::from_list_row(list_row) {
            Ok(res) => {
                list = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Update list
        // Update title
        match update_list_input.title {
            Some(title) => {
                list.title = title;
            },
            None => {}
        }
        // Update description
        list.description = update_list_input.description;
        // Update color hex
        match update_list_input.color_hex {
            Some(color_hex) => {
                match list.set_color_hex(color_hex) {
                    Ok(_) => {},
                    Err(err) => {
                        return Err(graphql_error_translate(
                            constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                            err
                        ));
                    }
                }
            },
            None => {}
        }
        // Convert updated list back to list row
        let updated_list_row: models::database::ListRow;
        match models::database::ListRow::from_list(list) {
            Ok(res) => {
                updated_list_row = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Execute Update
        match diesel::update(dsl::lists.filter(dsl::uuid.eq(updated_list_row.uuid.clone())))
            .set((
                dsl::title.eq(updated_list_row.title.clone()),
                dsl::description.eq(updated_list_row.description.clone()),
                dsl::color_hex.eq(updated_list_row.color_hex.clone())
            )).execute(conn) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Update creation information and return updated list row on success
        return match CreationInformationService::update_creation_information(
            conn,
            &updated_list_row.creation_information_uuid,
            update_creation_information_input
        ) {
            Ok(_res) => {
                Ok(updated_list_row)
            },
            Err(err) => {
                Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ))
            }
        }
    }

    pub fn add_new_task(
        conn: &SqliteConnection,
        create_creation_information_input: models::graphql::CreateCreationInformationInput,
        create_task_input: models::graphql::CreateTaskInput
    ) -> FieldResult<models::database::TaskRow> {
        // Find the list row to update
        let list_row: models::database::ListRow;
        match ListService::get_list_by_uuid(&conn, &create_task_input.parent_list_uuid) {
            Ok(result) => {
                match result {
                    Some(found_list_row) => {
                        list_row = found_list_row;
                    },
                    None => {
                        return Err(
                            graphql_error_translate(
                                constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                                format!("List '{}' not found", create_task_input.parent_list_uuid.to_string())
                            )
                        );
                    }
                }
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Create list from list row
        let mut updated_list: models::List;
        match models::List::from_list_row(list_row) {
            Ok(res) => {
                updated_list = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Create task row from task input
        // Grab the uuid of the user who is updating the task before the input object is swallowed
        let last_updated_by_user_uuid = create_creation_information_input.creator_user_uuid.clone();
        let created_task_row: models::database::TaskRow;
        match TaskService::create_task(conn, create_creation_information_input, create_task_input) {
            Ok(task_row) => {
                created_task_row = task_row;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                ));
            }
        }
        // Create task from task row
        match models::Task::from_task_row(created_task_row.clone()) {
            Ok(task) => {
                let mut updated_task_uuids = updated_list.task_uuids.clone().unwrap_or_default();
                updated_task_uuids.push(task.uuid.clone());
                updated_list.task_uuids = Some(updated_task_uuids);
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_ADDED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Convert updated list back to list row
        let updated_list_row: models::database::ListRow;
        match models::database::ListRow::from_list(updated_list) {
            Ok(res) => {
                updated_list_row = res;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Execute Update
        match diesel::update(dsl::lists.filter(dsl::uuid.eq(updated_list_row.uuid.clone())))
        .set(dsl::task_uuids.eq(updated_list_row.task_uuids.clone()))
        .execute(conn) {
            Ok(_) => {},
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err.to_string()
                ));
            }
        }
        
        // Create update creation information input from new creation information input
        let update_creation_information_input = models::graphql::UpdateCreationInformationInput {
            last_updated_by_user_uuid,
        };
        // Update creation information and return updated list row on success
        match CreationInformationService::update_creation_information(
            conn,
            &updated_list_row.creation_information_uuid,
            update_creation_information_input
        ) {
            Ok(_res) => {
                return Ok(created_task_row);
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::LIST_NOT_UPDATED_ERROR_MESSAGE.to_string(),
                    err.message().to_string()
                )); 
            }
        }
    }
}
