use uuid::Uuid;

use crate::api::models::database::ListRow;
use crate::api::models::graphql::CreateListInput;
use crate::api::models::utilities::{parse_color_hex, parse_json_uuid_array};

pub struct List {
    pub uuid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub color_hex: Option<String>,
    pub task_uuids: Option<Vec<Uuid>>,
    pub parent_list_uuid: Option<Uuid>,
    pub sub_list_uuids: Option<Vec<Uuid>>,
    pub shared_with_user_uuids: Option<Vec<Uuid>>,
    pub creation_information_uuid: Uuid
}

impl List {
    pub fn from_create_list_input(input: CreateListInput, creation_information_uuid: Uuid) -> Result<List, String> {
        // Generate uuid for new list
        let uuid = Uuid::new_v4();
        // Parse color hex
        let color_hex: Option<String>;
        match input.color_hex {
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
                color_hex = None;
            }
        }
        // These make this exponentially large and should be broken up
        /*
        // Parse task uuids
        let task_uuids: Option<Vec<Uuid>>;
        match &self.task_uuids {
            Some(task_uuids_json) => {
                match parse_json_uuid_array(&task_uuids_json) {
                    Ok(res) => {
                        task_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                task_uuids = None;
            }
        }
        // Parse parent list uuid
        let parent_list_uuid: Option<Uuid>;
        match &self.parent_list_uuid {
            Some(string_parent_list_uuid) => {
                match Uuid::parse_str(&string_parent_list_uuid) {
                    Ok(res) => {
                        parent_list_uuid = Some(res);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                parent_list_uuid = None;
            }
        }
        // Parse sub list uuids
        let sub_list_uuids: Option<Vec<Uuid>>;
        match &self.sub_list_uuids {
            Some(sub_list_uuids_json) => {
                match parse_json_uuid_array(&sub_list_uuids_json) {
                    Ok(res) => {
                        sub_list_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                sub_list_uuids = None;
            }
        }
        // Parse shared with user uuids
        let shared_with_user_uuids: Option<Vec<Uuid>>;
        match &self.shared_with_user_uuids {
            Some(shared_with_user_uuids_json) => {
                match parse_json_uuid_array(&shared_with_user_uuids_json) {
                    Ok(res) => {
                        shared_with_user_uuids = Some(res);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            None => {
                shared_with_user_uuids = None;
            }
        }*/
        Ok(List {
            uuid,
            title: input.title,
            description: input.description,
            color_hex,
            task_uuids: None,
            parent_list_uuid: None,
            sub_list_uuids: None,
            shared_with_user_uuids: None,
            creation_information_uuid: creation_information_uuid
        })
    }

    pub fn from_list_row(list_row: ListRow) -> Result<List, String> {
        // Parse uuid
        let uuid: Uuid;
        match Uuid::parse_str(&list_row.uuid) {
            Ok(res) => {
                uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        // Parse color hex
        let color_hex: Option<String>;
        match list_row.color_hex {
            Some(res) => {
                match parse_color_hex(&res) {
                    Ok(parsed_color_hex) => {
                        color_hex = Some(parsed_color_hex);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                color_hex = None;
            }
        }
        // Parse task uuids
        let task_uuids: Option<Vec<Uuid>>;
        match list_row.task_uuids {
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_task_uuids) => {
                        task_uuids = Some(parsed_task_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                task_uuids = None;
            }
        }
        // Parse parent list uuid
        let parent_list_uuid: Option<Uuid>;
        match list_row.parent_list_uuid {
            Some(res) => {
                match Uuid::parse_str(&res) {
                    Ok(parsed_parent_list_uuid) => {
                        parent_list_uuid = Some(parsed_parent_list_uuid);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                parent_list_uuid = None;
            }
        }
        // Parse sub list uuids
        let sub_list_uuids: Option<Vec<Uuid>>;
        match list_row.sub_list_uuids {
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_sub_list_uuids) => {
                        sub_list_uuids = Some(parsed_sub_list_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                sub_list_uuids = None;
            }
        }
        // Parse shared with user uuids
        let shared_with_user_uuids: Option<Vec<Uuid>>;
        match list_row.shared_with_user_uuids {
            Some(res) => {
                match parse_json_uuid_array(&res) {
                    Ok(parsed_shared_with_user_uuids) => {
                        shared_with_user_uuids = Some(parsed_shared_with_user_uuids);
                    },
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            },
            None => {
                shared_with_user_uuids = None;
            }
        }
        // Parse creation information uuid
        let creation_information_uuid: Uuid;
        match Uuid::parse_str(&list_row.creation_information_uuid) {
            Ok(res) => {
                creation_information_uuid = res;
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
        Ok(List {
            uuid,
            title: list_row.title,
            description: list_row.description,
            color_hex,
            task_uuids,
            parent_list_uuid,
            sub_list_uuids,
            shared_with_user_uuids,
            creation_information_uuid
        })
    }

    pub fn set_color_hex(&mut self, color_hex: String) -> Result<(), String> {
        match parse_color_hex(&color_hex) {
            Ok(res) => {
                self.color_hex = Some(res);
                Ok(())
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}
