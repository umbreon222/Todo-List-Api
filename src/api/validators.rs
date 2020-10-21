use regex::Regex;
use uuid::Uuid;
use serde_json;

use crate::api::errors::ValidationError;

pub fn validate_uuid(uuid: &String) -> Result<(), ValidationError> {
    match Uuid::parse_str(uuid) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("Invalid UUID")),
    }
}

pub fn validate_color_hex(color_hex: &String) -> Result<(), ValidationError> {
    let pattern = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    if !pattern.is_match(&color_hex) {
        return Err(ValidationError::new("Invalid color hex"));
    }
    Ok(())
}

pub fn validate_json_uuid_array(json_uuids: &String) -> Result<(), ValidationError> {
    match serde_json::from_str::<Vec<String>>(&json_uuids) {
        Ok(uuids) => {
            for uuid in uuids {
                match self::validate_uuid(&uuid) {
                    Ok(_) => {},
                    Err(_) => return Err(ValidationError::new("Invalid UUID in list")),
                };
            }
        },
        Err(err) => {
            return Err(ValidationError::new(&err.to_string()))
        }
    };
    Ok(())
}