use std::str::FromStr;
use std::sync::{Arc, Mutex};

use salvo::prelude::*;
use serde_json::{json, Value};

use sightnet_core::collection::Collection;
use sightnet_core::document::Document;
use sightnet_core::field::{Field, FieldType, FieldValue};

use crate::api_error::ApiError;
use crate::api_result::ApiResult;
use crate::routes::{generate_fields_json, get_json_body};
use crate::routes::collection::{get_collection, get_collection_id};
use crate::routes::state::STATE;

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

pub async fn get_document<'a>(collection: Arc<Mutex<Collection>>, document_id: i32) -> Result<Arc<Mutex<Document>>, ApiError> {
    let collection = collection.clone();
    let collection = collection.lock().unwrap();
    let document = collection.get(document_id);

    if document.is_none() {
        return Err(ApiError::new(9, "There is no document with such id."));
    }

    Ok(Arc::new(Mutex::new(document.unwrap().clone())))
}

#[handler]
pub async fn info(req: &mut Request, res: &mut Response) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id).await?;
    let collection = collection.clone();
    let document_id = get_document_id(req).await?;
    let document = get_document(collection.clone(), document_id).await?;
    let document = document.clone();
    let document = document.lock().unwrap();
    let fields = &document.fields;
    let mut json_fields = generate_fields_json(fields);

    Ok(ApiResult::new(json!({
        "id": document_id,
        "fields": json_fields
    })))
}

#[handler]
pub async fn create(req: &mut Request, res: &mut Response) -> Result<ApiResult, ApiError> {
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

    for field in fields {
        document.push(field.0, FieldValue::from(field.1.as_str().unwrap().to_owned()));
    }

    collection.lock().unwrap().push(document, None);
    Ok(ApiResult::new(Value::Null))
}

#[handler]
pub async fn update(req: &mut Request, res: &mut Response) -> Result<ApiResult, ApiError> {
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
        let f_value = document.get_mut(field.0).unwrap();
        f_value.value_string = Some(field.1.as_str().unwrap().to_owned());
    }

    Ok(ApiResult::new(Value::Null))
}

#[handler]
pub async fn remove(req: &mut Request, res: &mut Response) -> Result<ApiResult, ApiError> {
    let collection_id = get_collection_id(req).await?;
    let collection = get_collection(collection_id.clone()).await?;
    let document_id = get_document_id(req).await?;
    get_document(collection.clone(), document_id).await?;

    collection.clone().lock().unwrap().remove(document_id);
    Ok(ApiResult::new(Value::Null))
}
