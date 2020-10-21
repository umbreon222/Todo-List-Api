use uuid::Uuid;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::services::utilities::graphql_translate;
use crate::api::services::CreationInformationService;
use crate::api::schema;
use crate::api::models;

pub struct TaskService;

impl TaskService {
    pub fn all_tasks(conn: &SqliteConnection) -> FieldResult<Vec<models::Task>> {
        use schema::Tasks::dsl::*;

        graphql_translate(Tasks.load::<models::Task>(conn))
    }

    pub fn create_task(
        conn: &SqliteConnection,
        new_creation_information: models::CreateCreationInformationInput,
        new_task: models::CreateTaskInput
    ) -> FieldResult<models::Task> {
        use schema::Tasks::dsl::*;

        // Use creation information service to create a creation information object in db
        let created_creation_information: models::CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, new_creation_information) {
            Ok(created) => {
                created_creation_information = created;
            },
            Err(err) => return FieldResult::Err(err),
        };
        // Create new task row
        let uuid = Uuid::new_v4();
        let new_task = models::NewTask {
            UUID: &uuid.to_string(),
            Content: &new_task.content,
            Priority: new_task.priority.unwrap_or_default(),
            TagUUIDs: new_task.tag_uuids,
            IsComplete: new_task.is_complete.unwrap_or_default(),
            CreationInformationUUID: &created_creation_information.uuid
        };
        // Execute insertion
        let inserted = diesel::insert_into(schema::Tasks::table)
            .values(&new_task)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Tasks.filter(UUID.eq(uuid.to_string())).first::<models::Task>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Task not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_task_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<models::Task>> {
        use schema::Tasks::dsl::*;

        match Tasks.filter(UUID.eq(uuid)).first::<models::Task>(conn) {
            Ok(task) => Ok(Some(task)),
            Err(err) => match err {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(err)),
            },
        }
    }

    pub fn task_exists(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<bool> {
        use schema::Tasks::dsl::*;
        use diesel::select;
        use diesel::dsl::exists;
        
        return graphql_translate(select(exists(Tasks.filter(UUID.eq(uuid)))).get_result::<bool>(conn));
    }
}
