use std::{env, fs, os, thread};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use salvo::prelude::*;

use routes::collection;
use sightnet_core::file::File;

use crate::config::CFG;
use crate::routes::document;
use crate::routes::state::STATE;

mod routes;
mod api_error;
mod api_result;
mod config;

fn create_files() {
    let cfg = CFG.lock().unwrap();
    let path = PathBuf::from(cfg.directory.as_str()).join("db");

    fs::create_dir_all(path).expect("Failed at creating files");
}

fn load_collections() {
    let cfg = CFG.lock().unwrap();
    let path = PathBuf::from(cfg.directory.as_str()).join("db");
    let files = fs::read_dir(path).unwrap();

    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_name = file_path.to_str().unwrap();
        let collection_id = Path::new(file_name).file_stem().unwrap().to_str().unwrap();
        let collection = File::load(file_name).expect("Failed loading collections");

        println!("Loaded: {}", collection_id);
        println!("Size: {}", collection.len());

        STATE.lock().unwrap().collections.insert(collection_id.to_string(), Arc::new(Mutex::new(collection)));
    }
}

fn save_collections() {
    let collections = &STATE.lock().unwrap().collections;

    for collection in collections {
        collection.1.lock().unwrap().save().expect("Collection saving failed");
    }
}

#[tokio::main]
async fn main() {
    let collection_router =
        Router::with_path("col")
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
                        Router::with_path("doc")
                            .put(document::create)
                            .push(
                                Router::with_path("<document_id>")
                                    .get(document::info)
                                    .post(document::update)
                                    .delete(document::remove)
                            )
                    ),
            );

    create_files();
    load_collections();

    let background_thread = thread::spawn(move || {
        loop {
            save_collections();
            thread::sleep(Duration::from_secs(5))
        }
    });

    println!("Started at localhost:{}", 1551);

    Server::new(TcpListener::bind("127.0.0.1:1551"))
        .serve(collection_router)
        .await;
}
