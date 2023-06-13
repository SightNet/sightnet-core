use std::collections::HashMap;

use salvo::Request;
use serde_json::{json, Map, Value};

use sightnet_core::field::FieldValue;

use crate::api_error::ApiError;

pub mod state;
pub mod collection;
pub mod document;

pub async fn get_json_body(req: &mut Request) -> Result<Value, ApiError> {
    let data = String::from_utf8_lossy(req.payload().await.unwrap());
    let json = serde_json::from_str(data.as_ref());

    match json {
        Ok(json) => Ok(json),
        Err(_) => Err(ApiError::new(1, "You haven't provided json."))
    }
}

pub async fn get_query(req: &mut Request) -> Result<String, ApiError> {
    let query = req.query::<String>("q");

    match query {
        Some(query) => Ok(query),
        None => Err(ApiError::new(2, "You haven't provided query."))
    }
}

pub fn generate_fields_json(fields: &HashMap<String, FieldValue>) -> Value {
    let mut json_fields = json!({});

    for (name, value) in fields {
        json_fields[name] = match value {
            FieldValue::Int(value) => {
                Value::Number((*value).into())
            }
            FieldValue::Bool(value) => {
                Value::Bool(*value)
            }
            FieldValue::String(value, _) => {
                Value::String(value.into())
            }
        };
    }

    json_fields
}