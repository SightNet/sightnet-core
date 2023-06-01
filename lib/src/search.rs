use hashbrown::HashMap;

use crate::collection::Collection;
use crate::field::Field;
use crate::ranker::Ranker;
use crate::tokenizer::tokenize;

impl Collection {
    pub fn search(
        &self,
        query: &str,
        strict: bool,
        fields: Option<Vec<String>>,
        max: Option<usize>,
    ) -> Vec<(i32, f32)> {
        let max = max.unwrap_or(10);

        let terms = tokenize(query);

        let fields = match fields {
            Some(fields) => {
                let fields: Vec<&Field> =
                    fields.iter().map(|x| self.get_field(x).unwrap()).collect();

                fields
            }
            None => self.fields.iter().collect(),
        };

        let mut docs: HashMap<i32, f32> = HashMap::new();

        for term in &terms {
            for field in &fields {
                let ranks = Ranker::rank(term, strict, self, field);

                for rank in &ranks {
                    let e = docs.entry(*rank.0);
                    *e.or_default() += rank.1;
                }
            }
        }

        let mut sorted_docs: Vec<_> = docs.into_iter().collect();
        sorted_docs.sort_by(|x, y| y.1.total_cmp(&x.1));

        if sorted_docs.len() > max {
            return sorted_docs[0..max].to_vec();
        }

        sorted_docs
    }
}
