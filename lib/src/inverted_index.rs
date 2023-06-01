use std::collections::HashMap;

use bincode::{Decode, Encode};

use crate::term::Term;

#[derive(Debug, Clone, Encode, Decode)]
pub struct InvertedIndex {
    index: HashMap<Term, Vec<i32>>,
}

impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        InvertedIndex {
            index: HashMap::new(),
        }
    }

    pub fn push(&mut self, token: Term, id: i32) {
        let e = self.index.entry(token);
        e.or_default().push(id);
    }

    pub fn get(&self, term: &Term) -> Option<&Vec<i32>> {
        self.index.get(term)
    }
}
