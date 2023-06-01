use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;

use bincode::{Decode, Encode};

use crate::document::Document;
use crate::field::{Field, FieldValue};
use crate::file::File;
use crate::inverted_index::InvertedIndex;

#[derive(Encode, Decode)]
pub struct Collection {
    pub documents: HashMap<i32, Document>,
    pub fields: Vec<Field>,
    pub file_name: Option<String>,
    pub(crate) last_index: i32,
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            documents: HashMap::default(),
            fields: Vec::default(),
            file_name: None,
            last_index: 0,
        }
    }

    pub fn push_field(&mut self, name: &str, value: FieldValue) {
        let name = name.to_string();
        let inverted_index = InvertedIndex::new();

        self.fields.push(Field {
            name,
            value,
            inverted_index,
        });
    }

    pub fn commit(&mut self) {
        for i in 0..self.fields.len() {
            for j in i..self.len() {
                let doc = self.documents.get_mut(&(j as i32)).unwrap();
                let value = doc.process_field(self.fields[i].name.as_str());

                if let Some(value) = value {
                    if let FieldValue::String(_, tokens) = value {
                        let field = &mut self.fields[i];

                        for token in tokens {
                            field.inverted_index.push(token.clone(), j as i32);
                        }
                    }
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

    pub fn remove(&mut self, document_id: i32) {
        self.documents.remove(&document_id);
    }

    pub fn get(&self, id: i32) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn get_mut(&mut self, id: i32) -> Option<&mut Document> {
        self.documents.get_mut(&id)
    }

    pub fn get_field(&self, name: &String) -> Option<&Field> {
        self.fields.iter().find(|x| x.name == *name)
    }


    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }


    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn iter(&self) -> Iter<'_, i32, Document> {
        return self.documents.iter();
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, i32, Document> {
        return self.documents.iter_mut();
    }

    pub fn load(&mut self) -> Result<(), std::io::Error> {
        *self = File::load(self.file_name.clone().unwrap().as_str())?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        File::save(self, self.file_name.clone().unwrap().as_str())
    }
}

impl Default for Collection {
    fn default() -> Self {
        Collection::new()
    }
}
