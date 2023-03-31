use hashbrown::HashMap;

use crate::field::FieldValue;
use crate::tokenizer::tokenize;

#[derive(Debug, Eq, Clone)]
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
        let field_value = self.get_mut(field_name);

        if let Some(field_value) = field_value {
            let tokens = tokenize(field_value.as_string().unwrap().as_str());
            field_value.value_tokens = Some(tokens);
            Some(field_value)
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
