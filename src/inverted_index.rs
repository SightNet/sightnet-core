use hashbrown::HashMap;

use crate::term::Term;

#[derive(Debug)]
pub struct InvertedIndex {
    pub field_name: String,
    index: HashMap<Term, Vec<i32>>,
}

impl InvertedIndex {
    pub fn new(field_name: String) -> InvertedIndex {
        InvertedIndex {
            field_name,
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
