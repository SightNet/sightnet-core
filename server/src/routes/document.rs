use std::sync::{Arc, Mutex};

use salvo::prelude::*;
use serde_json::json;

use sightnet_core::collection::Collection;
use sightnet_core::document::Document;
use sightnet_core::field::FieldValue;

use crate::api_error::ApiError;
use crate::api_result::ApiResult;
use crate::routes::{generate_fields_json, get_json_body};
use crate::routes::collection::{get_collection, get_collection_id};

pub async fn get_document_id(req: &mut Request) -> Result<i32, ApiError> {
    let document_id = req.param::<String>("document_id");

    if document_id.is_none() {
        return Err(ApiError::new(7, "You haven't provided id."));
    }

    let document_id = document_id.unwrap();
    let document_id = document_id.parse::<i32>();

    if document_id.is_err() {
        return Err(ApiError::new(8, "You have provided id, but it is not integer."));
    }

    Ok(document_id.unwrap())
}

pub async fn get_document(collection: Arc<Mutex<Collection>>, document_id: i32) -> Result<Arc<Mutex<Document>>, ApiError> {
    let collection = collection.lock().unwrap();
    let document = collection.get(document_id);

    if document.is_none() {
        return Err(ApiError::new(9, "There is no document with such id."));
    }

    Ok(Arc::new(Mutex::new(document.unwrap().clone())))
}

#[handler]
pub async fn info(req: &mut Request) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id).await?;
    let collection = collection.clone();
    let document_id = get_document_id(req).await?;
    let document = get_document(collection.clone(), document_id).await?;
    let document = document.lock().unwrap();
    let fields = &document.fields;
    let mut json = generate_fields_json(fields);
    json["id"] = json!(document_id);

    Ok(ApiResult::new(Some(json)))
}

#[handler]
pub async fn create(req: &mut Request) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id.clone()).await?;
    let collection = collection.clone();

    let json = get_json_body(req).await?;
    let fields = json.as_object();

    if fields.is_none() {
        return Err(ApiError::new(12, "You haven't provided data."));
    }

    let mut document = Document::new();
    let fields = fields.unwrap();

    for collection_field in &collection.lock().unwrap().fields{
        if !fields.iter().any(|x| *x.0 == collection_field.name) {
            return Err(ApiError::new(20, "You haven't all fields."));
        }
    }

    for (name, value) in fields {
        let collection_fields = &collection.lock().unwrap().fields;
        let field_type = &collection_fields.iter().find(|x| x.name == *name).unwrap().value;
        let field_value = match field_type {
            FieldValue::Int(_) => {
                FieldValue::Int(value.as_i64().unwrap())
            }
            FieldValue::Bool(_) => {
                FieldValue::Bool(value.as_bool().unwrap())
            }
            FieldValue::String(_, _) => {
                FieldValue::String(value.as_str().unwrap().into(), vec![])
            }
        };

        document.push(name, field_value);
    }

    collection.lock().unwrap().push(document, None);
    Ok(ApiResult::new(None))
}

#[handler]
pub async fn update(req: &mut Request) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id.clone()).await?;
    let collection = collection.clone();
    let document_id = get_document_id(req).await?;
    let document = get_document(collection, document_id).await?;
    let document = document.clone();

    let json = get_json_body(req).await?;
    let fields = json.as_object();

    if fields.is_none() {
        return Err(ApiError::new(12, "You haven't provided data."));
    }

    let fields = fields.unwrap();

    for field in fields {
        let mut document = document.lock().unwrap();
        let field_value = document.get_mut(field.0).unwrap();

        match field_value {
            FieldValue::Int(value) => {
                *value = field.1.as_i64().unwrap()
            }
            FieldValue::Bool(value) => {
                *value = field.1.as_bool().unwrap()
            }
            FieldValue::String(value, _) => {
                *value = field.1.as_str().unwrap().to_string()
            }
        };
    }

    Ok(ApiResult::new(None))
}

#[handler]
pub async fn remove(req: &mut Request) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id.clone()).await?;
    let document_id = get_document_id(req).await?;
    get_document(collection.clone(), document_id).await?;

    collection.clone().lock().unwrap().remove(document_id);
    Ok(ApiResult::new(None))
}
