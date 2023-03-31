use std::cell::RefCell;
use std::rc::Rc;

use hashbrown::HashMap;

use crate::collection::Collection;
use crate::field::{Field, FieldType};
use crate::ranker::Ranker;
use crate::tokenizer::tokenize;

impl Collection {
    pub fn search(&self, query: &str, fields: Vec<String>, max: Option<usize>) -> Vec<(i32, f32)> {
        let terms = tokenize(query);
        let fields: Vec<Rc<RefCell<Field>>> = fields
            .iter()
            .map(|x| Rc::clone(self.get_field(x).unwrap()))
            .collect();

        let mut docs: HashMap<i32, f32> = HashMap::new();

        for term in &terms {
            for field in &fields {
                let ranks = Ranker::rank(term, &self, Rc::clone(field));

                for rank in &ranks {
                    let e = docs.entry(*rank.0);
                    *e.or_default() += rank.1;
                }
            }
        }

        let mut sorted_docs: Vec<_> = docs.into_iter().collect();
        sorted_docs.sort_by(|x, y| y.1.total_cmp(&x.1));

        if let Some(max) = max {
            return sorted_docs[0..max].to_vec();
        }

        sorted_docs
    }
}
