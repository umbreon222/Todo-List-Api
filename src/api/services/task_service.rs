use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

use crate::api::constants;
use crate::api::{models, schema};
use schema::tasks::dsl;
use crate::api::services::CreationInformationService;
use crate::api::services::utilities::{graphql_translate, graphql_error_translate};

pub struct TaskService;

impl TaskService {
    pub fn all_tasks(conn: &SqliteConnection) -> FieldResult<Vec<models::database::TaskRow>> {
        graphql_translate(dsl::tasks.load::<models::database::TaskRow>(conn))
    }

    pub fn create_task(
        conn: &SqliteConnection,
        create_creation_information_input: models::graphql::CreateCreationInformationInput,
        create_task_input: models::graphql::CreateTaskInput
    ) -> FieldResult<models::database::TaskRow> {
        // Use creation information service to create a creation information object in db
        let creation_information: models::CreationInformation;
        match CreationInformationService::create_creation_information(
            conn,
            create_creation_information_input
        ) {
            Ok(creation_information_row) => {
                match models::CreationInformation::from_creation_information_row(creation_information_row) {
                    Ok(res) => {
                        creation_information = res;
                    },
                    Err(err) => {
                        return Err(graphql_error_translate(
                            constants::TASK_NOT_CREATED_ERROR_MESSAGE.to_string(),
                            err
                        ));
                    }
                }
            },
            Err(err) => return Err(err)
        };
        // Parse create task input
        let new_task: models::Task;
        match create_task_input.create_task(&creation_information.uuid) {
            Ok(task) => {
                new_task = task;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Create new task row
        let new_task_row: models::database::TaskRow;
        match models::database::TaskRow::from_task(new_task) {
            Ok(task_row) => {
                new_task_row = task_row;
            },
            Err(err) => {
                return Err(graphql_error_translate(
                    constants::TASK_NOT_CREATED_ERROR_MESSAGE.to_string(),
                    err
                ));
            }
        }
        // Execute insertion
        match diesel::insert_into(schema::tasks::table)
            .values(&new_task_row)
            .execute(conn) {
                Ok(_) => {},
                Err(err) => {
                    return Err(graphql_error_translate(
                        constants::TASK_NOT_CREATED_ERROR_MESSAGE.to_string(),
                        err.to_string()
                    ));
                }
            }
        // Return error or newly inserted row via UUID look up
        match TaskService::get_task_by_uuid(&conn, &new_task_row.uuid) {
            Ok(res) => {
                match res {
                    Some(found) => Ok(found),
                    None => {
                        let error_details = format!(
                            "Couldn't find task '{}' after insert",
                            &new_task_row.uuid
                        );
                        return Err(graphql_error_translate(
                            constants::INTERNAL_ERROR.to_string(),
                            error_details
                        ));
                    }
                }
            },
            Err(err) => Err(err)
        }
    }

    pub fn get_task_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::database::TaskRow>> {
        match dsl::tasks.filter(dsl::uuid.eq(uuid)).first::<models::database::TaskRow>(conn) {
            Ok(task) => Ok(Some(task)),
            Err(err) => match err {
                diesel::result::Error::NotFound => Ok(None),
                _ => Err(FieldError::from(err))
            }
        }
    }

    pub fn task_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(
            select(
                exists(dsl::tasks.filter(dsl::uuid.eq(uuid)))
            ).get_result::<bool>(conn)
        );
    }
}
