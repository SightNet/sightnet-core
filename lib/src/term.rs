use bincode::{Decode, Encode};

#[derive(Debug, Eq, Clone, Hash, Encode, Decode)]
pub struct Term {
    pub value: String,
}

impl Term {
    pub fn new(value: String) -> Self {
        Term { value }
    }
}

impl From<&str> for Term {
    fn from(item: &str) -> Self {
        Self {
            value: item.to_string(),
        }
    }
}

impl From<String> for Term {
    fn from(item: String) -> Self {
        Self { value: item }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        self.value == other.value
    }
}