use std::str::FromStr;
use std::sync::{Arc, Mutex};

use salvo::prelude::*;
use serde_json::{json, Map, Value};
use serde_json::Value::Object;

use sightnet_core::collection::Collection;
use sightnet_core::field::{Field, FieldType};

use crate::api_error::ApiError;
use crate::api_result::ApiResult;
use crate::routes::{generate_fields_json, get_is_strict, get_json_body, get_max, get_query};
use crate::routes::state::STATE;

pub async fn get_collection_id(req: &mut Request) -> Result<String, ApiError> {
    let id = req.param::<String>("collection_id");

    if id.is_none() {
        return Err(ApiError::new(3, "You haven't provided collection id."));
    }

    Ok(id.unwrap())
}

pub async fn get_collection(id: String) -> Result<Arc<Mutex<Collection>>, ApiError> {
    let state = STATE.lock().unwrap();
    let collection = state.collections.get_key_value(id.as_str());

    if collection.is_none() {
        return Err(ApiError::new(4, "There is no collection with such name."));
    }

    let collection = collection.unwrap();
    Ok(collection.1.clone())
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

#[handler]
pub async fn info(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let collection = get_collection(id.clone()).await?;
    let collection = collection.clone();
    let collection = collection.lock().unwrap();
    let json_fields: Vec<Value> = collection.fields.iter().map(|x: &Field| {
        json!({
                x.name.as_str(): x.field_type.to_string()
            })
    }).collect();

    Ok(ApiResult::new(json!({
        "name": id,
        "fields": json_fields
    })))
}

#[handler]
pub async fn create(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let collection = get_collection(id.clone()).await;

    if collection.is_ok() {
        return Err(ApiError::new(5, "There is collection with such name."));
    }

    let json = get_json_body(req).await?;
    let fields = json.as_object();

    if fields.is_none() {
        return Err(ApiError::new(6, "You haven't provided fields."));
    }

    let fields = fields.unwrap();
    let mut collection = Collection::new();

    for field in fields {
        let value = FieldType::from_str(field.1.as_str().unwrap()).unwrap();
        collection.push_field(field.0, value);
    }

    STATE.lock().unwrap().collections.insert(id, Arc::new(Mutex::new(collection)));
    Ok(ApiResult::new(Value::Null))
}

#[handler]
pub async fn update(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let collection = get_collection(id.clone()).await?;

    dbg!(id);

    Ok(ApiResult::new(Value::Null))
}

#[handler]
pub async fn commit(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let collection = get_collection(id).await?;

    collection.clone().lock().unwrap().commit();
    Ok(ApiResult::new(Value::Null))
}

#[handler]
pub async fn search(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let query = get_query(req).await?;
    let is_strict = get_is_strict(req).await?;
    let max = get_max(req).await;
    let collection = get_collection(id.clone()).await?;
    let collection = collection.clone();
    let collection = collection.lock().unwrap();

    let results = collection.search(query.as_str(), is_strict, None, max);
    let mut json_results = Vec::new();

    for result in results {
        let document = collection.get(result.0);
        let json_fields = generate_fields_json(&document.unwrap().fields);
        json_results.push(json_fields);
    }

    Ok(ApiResult::new(Value::Array(json_results)))
}
