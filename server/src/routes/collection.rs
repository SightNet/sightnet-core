use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use salvo::prelude::*;
use serde_json::{json, Value};

use sightnet_core::collection::Collection;
use sightnet_core::field::{Field, FieldValue};

use crate::api_error::ApiError;
use crate::api_result::ApiResult;
use crate::config::{Cfg, CFG};
use crate::routes::{generate_fields_json, get_json_body, get_query};
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
    let collection = collection.lock().unwrap();
    let mut json = json!({});

    for field in &collection.fields {
        json[&field.name] = json!(field.value.to_string());
    }

    json["id"] = json!(id);
    Ok(ApiResult::new(Some(json)))
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
        let value = FieldValue::from_str(field.1.as_str().unwrap()).unwrap();
        collection.push_field(field.0, value);
    }

    let home = dirs::home_dir().unwrap();
    let path = home.join("db").join(format!("{}.bin", id.clone()));
    collection.file_name = Some(path.to_str().unwrap().into());

    STATE.lock().unwrap().collections.insert(id, Arc::new(Mutex::new(collection)));
    Ok(ApiResult::new(None))
}

#[handler]
pub async fn update(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;

    dbg!(id);

    Ok(ApiResult::new(None))
}

#[handler]
pub async fn commit(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let collection = get_collection(id).await?;

    collection.lock().unwrap().commit();
    Ok(ApiResult::new(None))
}

#[handler]
pub async fn search(req: &mut Request) -> Result<ApiResult, ApiError> {
    let id = get_collection_id(req).await?;
    let query = get_query(req).await?;
    let is_strict = get_is_strict(req).await?;
    let max = get_max(req).await;
    let collection = get_collection(id.clone()).await?;
    let collection = collection.lock().unwrap();

    let results = collection.search(query.as_str(), is_strict, None, max);
    let mut json_results = Vec::new();

    for result in results {
        let document = collection.get(result.0);
        let mut json = generate_fields_json(&document.unwrap().fields);
        json["id"] = json!(result.0);
        json["rank"] = json!(result.1);
        json_results.push(json);
    }

    Ok(ApiResult::new(Some(Value::Array(json_results))))
}
