use hashbrown::HashMap;

use crate::document::Document;
use crate::field::{Field, FieldType};
use crate::inverted_index::InvertedIndex;

pub struct Collection {
    pub documents: HashMap<i32, Document>,
    pub fields: Vec<Field>,
    pub(crate) last_index: i32,
}

impl Collection {
    pub fn default() -> Collection {
        Collection {
            documents: HashMap::default(),
            fields: Vec::default(),
            last_index: 0,
        }
    }

    pub fn push_field(&mut self, name: &str, field_type: FieldType) {
        let name = name.to_string();
        let inverted_index = InvertedIndex::new(name.clone());

        self.fields.push(Field {
            name,
            field_type,
            inverted_index,
        });
    }

    pub fn commit(&mut self) {
        for i in 0..self.fields.len() {
            for j in i..self.len() {
                let tokens;

                {
                    let doc = self.documents.get_mut(&(j as i32)).unwrap();
                    let field_value = doc.process_field(self.fields[i].name.as_str());
                    tokens = field_value.unwrap().value_tokens.as_ref().unwrap().clone();
                }

                let field = &mut self.fields[i];

                for token in tokens {
                    field.inverted_index.push(token.clone(), j as i32);
                }
            }
        }
    }

    pub fn push(&mut self, document: Document, index: Option<i32>) {
        if let Some(index) = index {
            if self.documents.get(&index).is_some() {
                panic!("There is document with the same index: {}", index);
            }

            self.documents.insert(index, document);
        } else {
            self.documents.insert(self.last_index, document);
            self.last_index += 1;
        }
    }

    pub fn get(&self, id: i32) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn get_field(&self, name: &String) -> Option<&Field> {
        self.fields.iter().find(|x| x.name == *name)
    }

    pub fn len(&self) -> usize {
        return self.documents.len();
    }

    pub fn iter(&self) -> hashbrown::hash_map::Iter<i32, Document> {
        return self.documents.iter();
    }

    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<i32, Document> {
        return self.documents.iter_mut();
    }
}
