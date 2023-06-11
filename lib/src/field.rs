use std::str::FromStr;

use bincode::{Decode, Encode};

use crate::inverted_index::InvertedIndex;
use crate::term::Term;

#[derive(Debug, Clone, Encode, Decode)]
pub enum FieldValue {
    Int(i64),
    Bool(bool),
    String(String, Vec<Term>),
}

impl FieldValue {
    fn as_string(&self) -> String {
        match self {
            FieldValue::Int(val) => val.to_string(),
            FieldValue::Bool(val) => val.to_string(),
            FieldValue::String(val, _) => val.into(),
        }
    }
}

impl ToString for FieldValue {
    fn to_string(&self) -> String {
        match self {
            FieldValue::Int(_) => "int".into(),
            FieldValue::Bool(_) => "bool".into(),
            FieldValue::String(_, _) => "string".into(),
        }
    }
}

impl FromStr for FieldValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(FieldValue::Int(0)),
            "bool" => Ok(FieldValue::Bool(false)),
            "string" => Ok(FieldValue::String("".into(), vec![])),
            _ => Err(()),
        }
    }
}

impl PartialEq for FieldValue {
    fn eq(&self, other: &FieldValue) -> bool {
        self.as_string() == other.as_string()
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
    pub inverted_index: InvertedIndex,
}

impl Field {}
