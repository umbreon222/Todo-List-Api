use chrono::prelude::*;
use uuid::Uuid;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult};

use crate::api::schema::*;
use crate::api::models::*;

const PASSWORD_HASH_SALT: &'static str = "pr3tz3ls&mcd0nalds_fr1es";

// User Service
pub struct UserService;

impl UserService {
    pub fn all_users(conn: &SqliteConnection) -> FieldResult<Vec<User>> {
        graphql_translate(Users::dsl::Users.load::<User>(conn))
    }

    pub fn create_user(
        conn: &SqliteConnection,
        new_user: CreateUserInput,
    ) -> FieldResult<User> {
        // Create new user row
        let uuid = Uuid::new_v4();
        // This may need to be done client side to avoid sending the real user's password over the network
        let mut salted_password = new_user.password.clone();
        salted_password.push_str(PASSWORD_HASH_SALT);
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(&salted_password);
        let password_hash = hasher.result_str();
        let new_user = NewUser {
            UUID: &uuid.to_string(),
            Username: &new_user.username,
            PasswordHash: &password_hash,
            Nickname: &new_user.nickname
        };
        // Execute insertion
        let inserted = diesel::insert_into(Users::table)
            .values(&new_user)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Users::dsl::Users.filter(Users::dsl::UUID.eq(uuid.to_string())).first::<User>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("User not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_user_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<User>> {
        match Users::dsl::Users.filter(Users::dsl::UUID.eq(uuid)).first::<User>(conn) {
            Ok(user) => Ok(Some(user)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }
}


// Creation Information Service
pub struct CreationInformationService;

impl CreationInformationService {
    pub fn all_creation_information(conn: &SqliteConnection) -> FieldResult<Vec<CreationInformationStruct>> {
        graphql_translate(CreationInformation::dsl::CreationInformation.load::<CreationInformationStruct>(conn))
    }

    pub fn create_creation_information(
        conn: &SqliteConnection,
        new_creation_information: CreateCreationInformationInput
    ) -> FieldResult<CreationInformationStruct> {
        // Create new creation information row
        let uuid = Uuid::new_v4();
        let current_time_string = Utc::now().to_rfc3339();
        let new_creation_information = NewCreationInformation {
            UUID: &uuid.to_string(),
            CreatorUserUUID: &new_creation_information.creator_user_uuid,
            CreationTime: &current_time_string,
            LastUpdatedByUserUUID: &new_creation_information.creator_user_uuid,
            LastUpdatedTime: &current_time_string
        };
        // Execute insertion
        let inserted = diesel::insert_into(CreationInformation::table)
            .values(&new_creation_information)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(CreationInformation::dsl::CreationInformation.filter(CreationInformation::dsl::UUID.eq(uuid.to_string())).first::<CreationInformationStruct>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Creation information not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_creation_information_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<CreationInformationStruct>> {
        match CreationInformation::dsl::CreationInformation.filter(CreationInformation::dsl::UUID.eq(uuid)).first::<CreationInformationStruct>(conn) {
            Ok(creation_information) => Ok(Some(creation_information)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }
}


// List Service
pub struct ListService;

impl ListService {
    pub fn all_lists(conn: &SqliteConnection) -> FieldResult<Vec<List>> {
        graphql_translate(Lists::dsl::Lists.load::<List>(conn))
    }

    pub fn create_list(
        conn: &SqliteConnection,
        new_creation_information: CreateCreationInformationInput,
        new_list: CreateListInput
    ) -> FieldResult<List> {
        // Use creation information service to create a creation information object in db
        let created_creation_information: CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, new_creation_information) {
            Ok(created) => {
                created_creation_information = created;
            },
            Err(err) => return Err(err),
        };
        // By default, the list being created will be shared with the creator.
        let mut updated_shared_with_user_uuids: Vec<String> = vec![created_creation_information.creator_user_uuid];
        // Append any additional users uuids that the creator wants to share with if need be
        match new_list.shared_with_user_uuids {
            Some(user_uuids_json) => {
                match serde_json::from_str(&user_uuids_json) {
                    Ok(mut user_uuids) => {
                        updated_shared_with_user_uuids.append(&mut user_uuids);
                    },
                    Err(err) => {
                        let err_string = err.to_string();
                        return FieldResult::Err(FieldError::new("List not created", graphql_value!({ "internal_error": err_string })))
                    },
                }
            },
            None => {},
        }
        // Convert the modified shared user ids list back to a json string
        let updated_shared_with_user_uuids_json = serde_json::to_string(&updated_shared_with_user_uuids)?;
        // Create new list row
        let uuid = Uuid::new_v4();
        let new_list = NewList {
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
        let inserted = diesel::insert_into(Lists::table)
            .values(&new_list)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Lists::dsl::Lists.filter(Lists::dsl::UUID.eq(uuid.to_string())).first::<List>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("List not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_list_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<List>> {
        match Lists::dsl::Lists.filter(Lists::dsl::UUID.eq(uuid)).first::<List>(conn) {
            Ok(list) => Ok(Some(list)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }

    pub fn add_task(
        conn: &SqliteConnection,
        list_uuid: String,
        task_uuid: String
    ) -> FieldResult<List> {
        // Grab id and current task uuids from the list
        let list_id: i32;
        let task_uuids: Option<String>;
        let query = Lists::dsl::Lists.select((Lists::dsl::ID, Lists::dsl::TaskUUIDs))
            .filter(Lists::dsl::UUID.eq(list_uuid.clone()))
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
                        return FieldResult::Err(FieldError::new("Task not added", graphql_value!({ "internal_error": err_string })))
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
        let updated = diesel::update(Lists::dsl::Lists.find(list_id))
            .set(Lists::dsl::TaskUUIDs.eq(updated_task_uuids_json))
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match updated {
            Ok(_size) => {
                graphql_translate(Lists::dsl::Lists.filter(Lists::dsl::UUID.eq(list_uuid)).first::<List>(conn))
            },
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Task not added", graphql_value!({ "internal_error": err_string })))
            },
        }
    }
}

// Task Service
pub struct TaskService;

impl TaskService {
    pub fn all_tasks(conn: &SqliteConnection) -> FieldResult<Vec<Task>> {
        graphql_translate(Tasks::dsl::Tasks.load::<Task>(conn))
    }

    pub fn create_task(
        conn: &SqliteConnection,
        new_creation_information: CreateCreationInformationInput,
        new_task: CreateTaskInput
    ) -> FieldResult<Task> {
        // Use creation information service to create a creation information object in db
        let created_creation_information: CreationInformationStruct;
        match CreationInformationService::create_creation_information(conn, new_creation_information) {
            Ok(created) => {
                created_creation_information = created;
            },
            Err(err) => return FieldResult::Err(err),
        };
        // Create new task row
        let uuid = Uuid::new_v4();
        let new_task = NewTask {
            UUID: &uuid.to_string(),
            Content: &new_task.content,
            Priority: new_task.priority.unwrap_or_default(),
            TagUUIDs: new_task.tag_uuids,
            IsComplete: new_task.is_complete.unwrap_or_default(),
            CreationInformationUUID: &created_creation_information.uuid
        };
        // Execute insertion
        let inserted = diesel::insert_into(Tasks::table)
            .values(&new_task)
            .execute(conn);
        // Return error or newly inserted row via UUID look up
        match inserted {
            Ok(_size) => graphql_translate(Tasks::dsl::Tasks.filter(Tasks::dsl::UUID.eq(uuid.to_string())).first::<Task>(conn)),
            Err(err) => {
                let err_string = err.to_string();
                FieldResult::Err(FieldError::new("Task not created", graphql_value!({ "internal_error": err_string })))
            },
        }
    }

    pub fn get_task_by_uuid(
        conn: &SqliteConnection,
        uuid: String,
    ) -> FieldResult<Option<Task>> {
        match Tasks::dsl::Tasks.filter(Tasks::dsl::UUID.eq(uuid)).first::<Task>(conn) {
            Ok(task) => Ok(Some(task)),
            Err(e) => match e {
                diesel::result::Error::NotFound => FieldResult::Ok(None),
                    _ => FieldResult::Err(FieldError::from(e)),
            },
        }
    }
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}