use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::io::Error;

use bincode::{Decode, Encode};

use crate::document::Document;
use crate::field::{Field, FieldValue};
use crate::file::File;
use crate::inverted_index::InvertedIndex;

#[derive(Default, Encode, Decode)]
pub struct Collection {
    pub documents: HashMap<i32, Document>,
    pub fields: Vec<Field>,
    pub file_name: Option<String>,
    pub(crate) last_index: i32,
}

impl Collection {
    pub fn new() -> Collection {
        Collection::default()
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
        //iterate over fields
        for field in self.fields.iter_mut() {
            //iterate over documents
            for doc in self.documents.iter_mut() {
                let value = doc.1.process_field(field.name.as_str());

                if let Some(FieldValue::String(_, tokens)) = value {
                    if let Some(tokens) = tokens {
                        for token in tokens {
                            field.inverted_index.push(token.clone(), *doc.0);
                        }
                    }
                }
            }
        }
    }

    pub fn push(&mut self, document: Document, index: Option<i32>) {
        match index {
            Some(index) => {
                let old_value = self.documents.insert(index, document);
                assert!(old_value.is_none(), "There is document with the same index: {}", index);
            }
            None => {
                self.documents.insert(self.last_index, document);
                self.last_index += 1;
            }
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

    pub fn load(&mut self) -> Result<(), Error> {
        assert!(self.file_name.is_some(), "You haven't passed file_name");
        *self = File::load(self.file_name.as_ref().unwrap())?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        assert!(self.file_name.is_some(), "You haven't passed file_name");
        File::save(self, self.file_name.as_ref().unwrap())
    }
}