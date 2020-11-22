use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::constants::{ERROR_DETAILS_KEY, TASK_NOT_CREATED_ERROR_MESSAGE};
use crate::api::{models, schema};
use schema::Tasks::dsl::*;
use crate::api::services::CreationInformationService;
use crate::api::services::utilities::graphql_translate;

pub struct TaskService;

impl TaskService {
    pub fn all_tasks(conn: &SqliteConnection) -> FieldResult<Vec<models::TaskRow>> {
        graphql_translate(Tasks.load::<models::TaskRow>(conn))
    }

    pub fn create_task(
        conn: &SqliteConnection,
        create_creation_information_input: models::CreateCreationInformationInput,
        create_task_input: models::CreateTaskInput
    ) -> FieldResult<models::TaskRow> {
        // Use creation information service to create a creation information object in db
        let creation_information: models::CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, create_creation_information_input) {
            Ok(creation_information_row) => {
                match creation_information_row.create_creation_information_struct() {
                    Ok(res) => {
                        creation_information = res;
                    },
                    Err(err) => {
                        return FieldResult::Err(FieldError::new(TASK_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
                    }
                }
            },
            Err(err) => return FieldResult::Err(err)
        };
        let new_task: models::Task;
        match create_task_input.create_task(&creation_information.uuid) {
            Ok(task) => {
                new_task = task;
            },
            Err(err) => {
                return FieldResult::Err(FieldError::new(TASK_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
            }
        }
        let new_task_row: models::NewTaskRow;
        match new_task.create_new_task_row() {
            Ok(task_row) => {
                new_task_row = task_row;
            },
            Err(err) => {
                return FieldResult::Err(FieldError::new(TASK_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err })));
            }
        }
        // Execute insertion
        let inserted = diesel::insert_into(schema::Tasks::table)
            .values(&new_task_row)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Tasks.filter(UUID.eq(new_task.uuid.to_string())).first::<models::TaskRow>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new(TASK_NOT_CREATED_ERROR_MESSAGE, graphql_value!({ ERROR_DETAILS_KEY: err_string })))
            }
        }
    }

    pub fn get_task_by_uuid(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<Option<models::TaskRow>> {
        match Tasks.filter(UUID.eq(uuid.clone())).first::<models::TaskRow>(conn) {
            Ok(task) => Ok(Some(task)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                _ => FieldResult::Err(FieldError::from(err))
            }
        }
    }

    pub fn task_exists(
        conn: &SqliteConnection,
        uuid: &String
    ) -> FieldResult<bool> {
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Tasks.filter(UUID.eq(uuid.clone())))).get_result::<bool>(conn));
    }
}
