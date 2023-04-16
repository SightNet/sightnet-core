use crate::inverted_index::InvertedIndex;
use crate::term::Term;

#[derive(Copy, Clone, Debug)]
pub enum FieldType {
    Int = 0,
    Bool = 1,
    String = 2,
}

impl FieldType {
    pub fn from_u32(value: u32) -> FieldType {
        match value {
            0 => FieldType::Int,
            1 => FieldType::Bool,
            2 => FieldType::String,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct FieldValue {
    pub value_int: Option<i64>,
    pub value_bool: Option<bool>,
    pub value_string: Option<String>,
    pub value_tokens: Option<Vec<Term>>,
}

impl FieldValue {
    pub fn new() -> FieldValue {
        FieldValue {
            value_int: None,
            value_bool: None,
            value_string: None,
            value_tokens: None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap());
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap() as i64);
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().parse::<i64>().unwrap());
        }

        None
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap() != 0);
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap());
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().parse::<bool>().unwrap());
        }

        None
    }

    pub fn as_string(&self) -> Option<String> {
        if self.value_int.is_some() {
            return Some(self.value_int.unwrap().to_string());
        } else if self.value_bool.is_some() {
            return Some(self.value_bool.unwrap().to_string());
        } else if self.value_string.is_some() {
            return Some(self.value_string.as_ref().unwrap().clone());
        }

        None
    }
}

impl From<i64> for FieldValue {
    fn from(value: i64) -> Self {
        let mut fv = FieldValue::new();
        fv.value_int = Some(value);

        fv
    }
}

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        let mut fv = FieldValue::new();
        fv.value_bool = Some(value);

        fv
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        let mut fv = FieldValue::new();
        fv.value_string = Some(value);

        fv
    }
}

impl PartialEq for FieldValue {
    fn eq(&self, other: &FieldValue) -> bool {
        self.as_string() == other.as_string()
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub inverted_index: InvertedIndex,
}

impl Field {}
