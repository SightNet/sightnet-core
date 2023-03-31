use std::cell::RefCell;
use std::collections::hash_map::{Iter, IterMut};
use std::rc::Rc;

use hashbrown::HashMap;

use crate::document::Document;
use crate::field::{Field, FieldType};
use crate::inverted_index::InvertedIndex;

pub struct Collection {
    pub documents: HashMap<i32, Document>,
    pub fields: Vec<Rc<RefCell<Field>>>,
    last_index: i32,
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

        self.fields.push(Rc::new(RefCell::new(Field {
            name,
            field_type,
            inverted_index,
        })));
    }

    pub fn commit(&mut self) {
        for i in 0..self.fields.len() {
            let field = Rc::clone(&self.fields[i]);

            for (id, doc) in self.iter_mut() {
                let field_value = doc.process_field(field.borrow().name.as_str());
                let tokens = field_value.unwrap().value_tokens.as_ref().unwrap();

                for token in tokens {
                    field.borrow_mut().inverted_index.push(token.clone(), *id);
                }
            }
        }
    }

    pub fn push(&mut self, document: Document) {
        self.documents.insert(self.last_index, document);
        self.last_index += 1;
    }

    pub fn push_custom_index(&mut self, document: Document, index: i32) {
        if self.documents.get(&index).is_some() {
            panic!("There is document with the same index: {}", index);
        }

        self.documents.insert(index, document);
    }

    pub fn get(&self, id: i32) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn get_field(&self, name: &String) -> Option<&Rc<RefCell<Field>>> {
        self.fields.iter().find(|x| x.borrow().name == *name)
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
