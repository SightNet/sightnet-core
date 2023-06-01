use hashbrown::HashMap;
use salvo::Request;
use serde_json::{Map, Value};

use sightnet_core::field::FieldValue;

use crate::api_error::ApiError;

pub mod state;
pub mod collection;
pub mod document;

pub async fn get_json_body(req: &mut Request) -> Result<Value, ApiError> {
    let data = String::from_utf8_lossy(req.payload().await.unwrap());
    let json = serde_json::from_str(data.as_ref());

    if json.is_err() {
        return Err(ApiError::new(1, "You haven't provided json."));
    }

    let json: Value = json.unwrap();
    Ok(json)
}

pub async fn get_query(req: &mut Request) -> Result<String, ApiError> {
    let query = req.query::<String>("q");

    if query.is_none() {
        return Err(ApiError::new(2, "You haven't provided query."));
    }

    Ok(query.unwrap())
}

pub async fn get_is_strict(req: &mut Request) -> Result<bool, ApiError> {
    let is_strict = req.query::<bool>("strict");

    if is_strict.is_none() {
        return Ok(false);
    }

    Ok(is_strict.unwrap())
}

pub async fn get_max(req: &mut Request) -> Option<usize> {
    req.param::<usize>("max")
}

pub fn generate_fields_json(fields: &HashMap<String, FieldValue>) -> Value {
    let mut json_fields = Value::Object(Map::new());

    for field in fields {
        if (field.1.value_int.is_some()) {
            json_fields[field.0] = Value::Number(field.1.value_int.unwrap().into());
        } else if (field.1.value_bool.is_some()) {
            json_fields[field.0] = Value::Bool(field.1.value_bool.unwrap().into());
        } else if (field.1.value_string.is_some()) {
            let val = field.1.value_string.clone().unwrap().into();
            json_fields[field.0] = Value::String(val);
        } else {
            //TODO: ERROR
        }
    }

    json_fields
}