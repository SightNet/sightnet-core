use std::collections::HashMap;
use std::iter::Map;
use std::str::FromStr;
use std::sync::Mutex;

use lazy_static::lazy_static;
use salvo::affix;
use salvo::prelude::*;
use salvo::routing::PathState;
use serde::__private::doc;
use serde::Serialize;
use serde_json::{json, Value};

use routes::collection;
use sightnet_core::collection::Collection;
use sightnet_core::field::{Field, FieldType};

use crate::routes::document;

mod routes;
mod api_error;
mod api_result;

/*async fn collection_filter(req: &mut Request, _: &mut PathState) -> bool {
    let id = req.param::<String>("id");

    if id.is_none() {
        // error(res, "You haven't provided id.");
        return false;
    }

    let id = id.unwrap();
    let state = STATE.lock().unwrap();
    let collection = state.collections.get_key_value(id.as_str());

    if collection.is_none() {
        // error(res, "There isn't collection with id.");
        return false;
    }

    return true;
}*/

#[tokio::main]
async fn main() {
    let collection_router =
        Router::with_path("collection")
            .push(
                Router::with_path("<collection_id>")
                    .get(collection::info)
                    .put(collection::create)
                    .patch(collection::update)
                    .push(
                        Router::with_path("search")
                            .get(collection::search)
                    )
                    .push(
                        Router::with_path("commit")
                            .get(collection::commit)
                    )
                    .push(
                        Router::with_path("documents")
                            .put(document::create)
                            .push(
                                Router::with_path("<document_id>")
                                    .get(document::info)
                                    .post(document::update)
                                    .delete(document::remove)
                            )
                    ),
            )
        ;

    Server::new(TcpListener::bind("127.0.0.1:1551"))
        .serve(collection_router)
        .await;
}
