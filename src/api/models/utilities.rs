use regex::Regex;
use uuid::Uuid;
use serde_json;

pub fn parse_color_hex(color_hex: &String) -> Result<String, String> {
    let pattern = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    if !pattern.is_match(color_hex) {
        return Err(format!("Invalid color hex '{}'", color_hex));
    }
    Ok(color_hex.to_string())
}

pub fn parse_json_uuid_array(json_uuids: &String) -> Result<Vec<Uuid>, String> {
    return match serde_json::from_str::<Vec<String>>(json_uuids) {
        Ok(uuid_strings) => {
            let mut uuids: Vec<Uuid> = vec![];
            for uuid_string in uuid_strings {
                match Uuid::parse_str(&uuid_string) {
                    Ok(res) => {
                        uuids.push(res);
                    },
                    Err(_) => {
                        return Err(format!("Invalid uuid '{}'", uuid_string));
                    }
                };
            }
            Ok(uuids)
        },
        Err(err) => {
            Err(err.to_string())
        }
    }
}
