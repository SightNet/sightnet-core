use hashbrown::HashMap;

use crate::collection::Collection;
use crate::field::{Field, FieldType};
use crate::term::Term;

// #[derive(Debug, Default)]
// pub struct Rank {
//     pub bm25: f32,
// }

pub struct Ranker {}

impl Ranker {
    fn idf(term: &Term, collection: &Collection, field: &Field) -> f32 {
        let documents_count = collection.len() as f32;
        let inverted_index = &field.inverted_index;
        let documents_with_word = inverted_index.get(term);

        if documents_with_word.is_none() {
            return 0 as f32;
        }

        let documents_with_word_count = documents_with_word.unwrap().len() as f32;

        ((documents_count - documents_with_word_count + 0.5f32)
            / (documents_with_word_count + 0.5f32)
            + 1f32)
            .ln()
    }

    fn bm25(term: &Term, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        let idf: f32 = Self::idf(term, collection, field);
        let k1: f32 = 1.2;
        let b: f32 = 0.75;
        let d: f32 = collection.len() as f32;

        let sum_of_tokens_count: usize = collection
            .iter()
            .map(|(_id, doc)| {
                let field_value = doc.get(field.inverted_index.field_name.as_str()).unwrap();
                field_value.value_tokens.as_ref().unwrap_or(&Vec::new()).len()
            })
            .sum();

        let avgdl: f32 = sum_of_tokens_count as f32 / d;
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.inverted_index.field_name.as_str()).unwrap();
            let freq: f32 = field_value
                .value_tokens
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|x| *x == term)
                .count() as f32;
            let bm25: f32 =
                idf * ((freq * (k1 + 1f32)) / (freq + k1 * (1f32 - b + b * (d / avgdl))));

            if bm25 == 0f32 {
                continue;
            }

            ranks.insert(id.clone(), bm25);
        }

        ranks
    }

    pub fn rank_int(term: &Term, strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.inverted_index.field_name.as_str()).unwrap();

            if field_value.value_int.unwrap_or_default().to_string() == term.value {
                ranks.insert(*id, 1f32);
            }
        }

        ranks
    }

    pub fn rank_bool(term: &Term, strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.inverted_index.field_name.as_str()).unwrap();

            if field_value.value_bool.unwrap_or_default().to_string() == term.value {
                ranks.insert(*id, 1f32);
            }
        }

        ranks
    }

    pub fn rank_string(term: &Term, strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        if !strict {
            return Self::bm25(term, collection, field);
        }

        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.inverted_index.field_name.as_str()).unwrap();

            if field_value.value_string.as_ref().unwrap() == &term.value {
                ranks.insert(*id, 1f32);
            }
        }

        ranks
    }

    pub fn rank(term: &Term, strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        match field.field_type {
            FieldType::Int => Self::rank_int(term, strict, collection, field),
            FieldType::Bool => Self::rank_bool(term, strict, collection, field),
            FieldType::String => Self::rank_string(term, strict, collection, field),
        }
    }
}
