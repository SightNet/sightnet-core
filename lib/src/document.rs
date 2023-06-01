use std::collections::HashMap;
use crate::field::FieldValue;
use crate::tokenizer::tokenize;

use bincode::{Encode, Decode};

#[derive(Debug, Eq, Clone, Encode, Decode)]
pub struct Document {
    pub fields: HashMap<String, FieldValue>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            fields: HashMap::new(),
        }
    }

    pub fn push(&mut self, field_name: &str, field_value: FieldValue) {
        let entry = self.fields.get_mut(field_name);

        if let Some(entry) = entry {
            *entry = field_value;
        } else {
            self.fields.insert(field_name.to_string(), field_value);
        }
    }

    pub fn get(&self, field_name: &str) -> Option<&FieldValue> {
        self.fields.get(field_name)
    }

    pub fn get_mut(&mut self, field_name: &str) -> Option<&mut FieldValue> {
        self.fields.get_mut(field_name)
    }

   pub fn process_field(&mut self, field_name: &str) -> Option<&mut FieldValue> {
    if let Some(field_value) = self.get_mut(field_name) {
        if let Some(str_value) = field_value.as_string() {
            let tokens = tokenize(str_value.as_str());
            field_value.value_tokens = Some(tokens);
            Some(field_value)
        } else {
            None
        }
    } else {
        None
    }
}
}

impl PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        self.fields == other.fields
    }
}
