use std::collections::HashMap;

use crate::collection::Collection;
use crate::field::{Field, FieldValue};
use crate::term::Term;

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
                let field_value = doc.get(field.name.as_str()).unwrap();

                if let FieldValue::String(_, tokens) = field_value {
                    if let Some(tokens) = tokens {
                        return tokens.len()
                    }
                }
                0
            })
            .sum();

        let avgdl: f32 = sum_of_tokens_count as f32 / d;
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.name.as_str()).unwrap();

            if let FieldValue::String(_, tokens) = field_value {
                if let Some(tokens) = tokens {
                    let freq: f32 = tokens
                        .iter()
                        .filter(|x| *x == term)
                        .count() as f32;
                    let bm25: f32 =
                        idf * ((freq * (k1 + 1f32)) / (freq + k1 * (1f32 - b + b * (d / avgdl))));

                    if bm25 == 0f32 {
                        continue;
                    }

                    ranks.insert(*id, bm25);
                }
            }
        }

        ranks
    }

    pub fn rank_int(term: &Term, _strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.name.as_str()).unwrap();

            if let FieldValue::Int(value) = field_value {
                if value.to_string() == term.value {
                    ranks.insert(*id, 1f32);
                }
            }
        }

        ranks
    }

    pub fn rank_bool(term: &Term, _strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        let mut ranks = HashMap::new();

        for (id, doc) in collection.iter() {
            let field_value = doc.get(field.name.as_str()).unwrap();

            if let FieldValue::Bool(value) = field_value {
                if value.to_string() == term.value {
                    ranks.insert(*id, 1f32);
                }
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
            let field_value = doc.get(field.name.as_str()).unwrap();

            if let FieldValue::String(value, _) = field_value {
                if value == &term.value {
                    ranks.insert(*id, 1f32);
                }
            }
        }

        ranks
    }

    pub fn rank(term: &Term, strict: bool, collection: &Collection, field: &Field) -> HashMap<i32, f32> {
        match field.value {
            FieldValue::Int(_) => Self::rank_int(term, strict, collection, field),
            FieldValue::Bool(_) => Self::rank_bool(term, strict, collection, field),
            FieldValue::String(_, _) => Self::rank_string(term, strict, collection, field),
        }
    }
}
