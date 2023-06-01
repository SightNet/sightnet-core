use std::collections::HashMap;

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

pub fn generate_fields_json(fields: &HashMap<String, FieldValue>) -> Value {
    let mut json_fields = Value::Object(Map::new());

    for (name, value) in fields {
        if let FieldValue::Int(value) = value {
            json_fields[name] = Value::Number((*value).into());
        } else if let FieldValue::Bool(value) = value {
            json_fields[name] = Value::Bool(*value);
        } else if let FieldValue::String(value, _tokens) = value {
            json_fields[name] = Value::String(value.into());
        } else {
            panic!("WTF?!")
        }
    }

    json_fields
}