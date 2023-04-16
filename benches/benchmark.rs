use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use std::env::args;
use std::fs;

#[path = "../src/collection.rs"]
mod collection;

#[path = "../src/document.rs"]
mod document;

#[path = "../src/term.rs"]
mod term;

#[path = "../src/field.rs"]
mod field;

#[path = "../src/file.rs"]
mod file;

#[path = "../src/inverted_index.rs"]
mod inverted_index;

#[path = "../src/ranker.rs"]
mod ranker;

#[path = "../src/search.rs"]
mod search;

#[path = "../src/tokenizer.rs"]
mod tokenizer;

use crate::collection::Collection;
use crate::document::Document;
use crate::field::{FieldType, FieldValue};
use crate::file::File;
use crate::tokenizer::tokenize;

lazy_static! {
    static ref CORPUS: String =
        fs::read_to_string("./corpora/20140615-wiki-en_000000.txt").unwrap();
    static ref FIELD_NAME: &'static str = "text";
}

fn load_sample_corpus<'a>() -> Collection {
    let mut collection = Collection::default();
    collection.push_field(*FIELD_NAME, FieldType::String);

    for line in CORPUS.lines() {
        let mut document = Document::new();
        let mut field_value = FieldValue::new();
        field_value.value_string = Some(line.to_string());

        document.push(*FIELD_NAME, field_value);
        collection.push(document);
    }

    collection
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine");
    let mut collection = load_sample_corpus();

    group.sample_size(10);

    group.bench_function("Tokenize", |b| b.iter(|| tokenize(CORPUS.as_str())));
    group.bench_function("Load collection from memory", |b| {
        b.iter(|| load_sample_corpus())
    });
    group.bench_function("Index", |b| {
        b.iter(|| {
            collection.commit();
        })
    });
    group.bench_function("Search - 1 word", |b| {
        b.iter(|| {
            collection.search("sample", Some(vec![FIELD_NAME.to_string()]), Some(5));
        })
    });
    group.bench_function("Save", |b| {
        b.iter(|| {
            File::save(&collection, "out.bin").unwrap();
        })
    });
    group.bench_function("Load", |b| {
        b.iter(|| {
            File::load("out.bin").unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
