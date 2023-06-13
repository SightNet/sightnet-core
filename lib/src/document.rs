use std::collections::HashMap;

use bincode::{Decode, Encode};

use crate::field::FieldValue;
use crate::tokenizer::tokenize;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Document {
    pub fields: HashMap<String, FieldValue>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, field_name: &str, field_value: FieldValue) {
        let entry = self.fields.get_mut(field_name);

        match entry {
            Some(entry) => {
                *entry = field_value;
            }
            None => {
                self.fields.insert(field_name.to_string(), field_value);
            }
        }
    }

    pub fn get(&self, field_name: &str) -> Option<&FieldValue> {
        self.fields.get(field_name)
    }

    pub fn get_mut(&mut self, field_name: &str) -> Option<&mut FieldValue> {
        self.fields.get_mut(field_name)
    }

    pub fn process_field(&mut self, name: &str) -> Option<&mut FieldValue> {
        match self.get_mut(name) {
            Some(field_value) => {
                if let FieldValue::String(value, tokens) = field_value {
                    *tokens = Some(tokenize(value.clone().as_str()));
                    return Some(field_value);
                }
                None
            }
            None => None
        }
    }
}

impl PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        self.fields == other.fields
    }
}
