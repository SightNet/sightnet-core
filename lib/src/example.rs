use std::fs;
use std::str::FromStr;

use sightnet_core::collection::Collection;
use sightnet_core::document::Document;
use sightnet_core::field::{FieldValue};
use sightnet_core::file::File;

pub fn main() {
    let mut collection = Collection::new();
    collection.push_field("title", FieldValue::String("".into(), vec![]));

    let list = fs::read_to_string("corpora/corpus.txt").unwrap();
    let corpus: Vec<&str> = list.split("\n").collect();

    dbg!(corpus.len());

    for i in corpus {
        let mut doc = Document::new();
        doc.push("title", FieldValue::String(i.into(), vec![]));
        collection.push(doc, None);
    }

    collection.commit();

    // let collection = File::load("out.bin").unwrap();

    println!("{:#?}", collection.search("dream", false, None, None));
    File::save(&collection, "out.bin").unwrap();
}
