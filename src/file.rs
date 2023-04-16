extern crate fs2;

use std::io::{Read, Write};
use std::time::Duration;
use std::{fs, io, thread};

use fs2::FileExt;

use crate::collection::Collection;
use crate::document::Document;
use crate::field::{FieldType, FieldValue};

#[derive(Clone, Debug)]
enum TokenType {
    CollectionFieldName = 0,
    CollectionFieldType = 1,
    DocumentId = 2,
    DocumentFieldsStart = 3,
    DocumentFieldsEnd = 4,
    DocumentFieldName = 5,
    DocumentFieldValue = 6,
}

impl TokenType {
    fn from_u32(value: u32) -> TokenType {
        match value {
            0 => TokenType::CollectionFieldName,
            1 => TokenType::CollectionFieldType,
            2 => TokenType::DocumentId,
            3 => TokenType::DocumentFieldsStart,
            4 => TokenType::DocumentFieldsEnd,
            5 => TokenType::DocumentFieldName,
            6 => TokenType::DocumentFieldValue,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

pub struct File {}

impl File {
    fn write(bytes: &mut Vec<u8>, val: &str) {
        bytes.append(&mut usize::to_be_bytes(val.as_bytes().len()).to_vec());
        bytes.append(&mut val.as_bytes().to_vec());
    }
    fn write_number(bytes: &mut Vec<u8>, val: i32) {
        let val = val.to_string();
        File::write(bytes, val.as_str());
    }
    fn write_field_value(bytes: &mut Vec<u8>, val: &FieldValue) {
        let val = val.as_string().unwrap();
        File::write(bytes, val.as_str());
    }
    fn write_field_type(bytes: &mut Vec<u8>, val: FieldType) {
        File::write_number(bytes, val as i32);
    }
    fn write_token_type(bytes: &mut Vec<u8>, val: TokenType) {
        File::write_number(bytes, val as i32);
    }
    fn write_string(bytes: &mut Vec<u8>, val: &String) {
        File::write(bytes, val.as_str());
    }

    pub fn save(collection: &Collection, file_name: &str) -> Result<(), io::Error> {
        let mut bytes = Vec::new();

        for field in collection.fields.iter() {
            File::write_token_type(&mut bytes, TokenType::CollectionFieldName);
            File::write_string(&mut bytes, &field.name);
            File::write_token_type(&mut bytes, TokenType::CollectionFieldType);
            File::write_field_type(&mut bytes, field.field_type);
        }

        for (id, doc) in collection.documents.iter() {
            File::write_token_type(&mut bytes, TokenType::DocumentId);
            File::write_number(&mut bytes, *id);
            File::write_token_type(&mut bytes, TokenType::DocumentFieldsStart);
            File::write(&mut bytes, "");

            for (name, val) in &doc.fields {
                File::write_token_type(&mut bytes, TokenType::DocumentFieldName);
                File::write(&mut bytes, name);
                File::write_token_type(&mut bytes, TokenType::DocumentFieldValue);
                File::write_field_value(&mut bytes, val);
            }

            File::write_token_type(&mut bytes, TokenType::DocumentFieldsEnd);
            File::write(&mut bytes, "");
        }

        let mut file = fs::File::create(file_name)?;

        file.lock_exclusive()?;
        file.write_all(&bytes)?;
        file.unlock()?;

        Ok(())
    }

    fn next(bytes: &mut Vec<u8>, i: &mut i32) -> String {
        let i_usize = *i as usize;
        let len_bytes = bytes[i_usize..i_usize + 8].to_vec();
        let len = usize::from_be_bytes(len_bytes.try_into().unwrap());
        *i += 8;

        let i_usize = *i as usize;
        let value_bytes = bytes[i_usize..i_usize + len].to_vec();
        let value = String::from_utf8(value_bytes).unwrap();
        *i += len as i32;

        return value;
    }

    fn next_pair(bytes: &mut Vec<u8>, i: &mut i32) -> (TokenType, String) {
        let token_type = File::next(bytes, i);
        let token_value = File::next(bytes, i);

        let token_type_parsed = TokenType::from_u32(token_type.parse::<u32>().unwrap());

        return (token_type_parsed, token_value);
    }

    pub fn load(file_name: &str) -> Result<Collection, io::Error> {
        let mut collection = Collection::default();
        let mut bytes: Vec<u8> = Vec::new();

        {
            let mut file = fs::File::open(file_name)?;

            file.lock_exclusive()?;
            file.read_to_end(&mut bytes)?;
            file.unlock()?;
        }

        let mut i = 0;

        let mut temp_field_name: Option<String> = None;
        let mut temp_document_id: Option<i32> = None;
        let mut temp_document: Option<Document> = None;

        while i < bytes.len() as i32 {
            let (token_type, val) = File::next_pair(&mut bytes, &mut i);

            match token_type {
                TokenType::CollectionFieldName => {
                    temp_field_name = Some(val);
                }
                TokenType::CollectionFieldType => {
                    let field_name = temp_field_name
                        .clone()
                        .expect("FieldName should be defined before FieldType!");
                    let field_type = FieldType::from_u32(val.parse::<u32>().unwrap());

                    collection.push_field(field_name.as_str(), field_type);
                }
                TokenType::DocumentId => {
                    temp_document_id = Some(val.parse::<i32>().unwrap());
                }
                TokenType::DocumentFieldsStart => {
                    temp_document = Some(Document::new());
                }
                TokenType::DocumentFieldsEnd => {
                    let id = temp_document_id.unwrap();
                    collection.push(temp_document.clone().unwrap(), Some(id));
                }
                TokenType::DocumentFieldName => {
                    temp_field_name = Some(val);
                }
                TokenType::DocumentFieldValue => {
                    let field_name = temp_field_name
                        .clone()
                        .expect("FieldName should be defined before FieldType!");
                    let field = collection
                        .get_field(&field_name)
                        .expect("Collection's fields should be defined before Documents!");
                    let field_type = field.field_type;

                    let field_value: FieldValue = match field_type {
                        FieldType::Int => FieldValue {
                            value_int: Some(val.parse::<i64>().unwrap()),
                            value_bool: None,
                            value_string: None,
                            value_tokens: None,
                        },
                        FieldType::Bool => FieldValue {
                            value_int: None,
                            value_bool: Some(val.parse::<bool>().unwrap()),
                            value_string: None,
                            value_tokens: None,
                        },
                        FieldType::String => FieldValue {
                            value_int: None,
                            value_bool: None,
                            value_string: Some(val),
                            value_tokens: None,
                        },
                    };

                    temp_document
                        .as_mut()
                        .unwrap()
                        .push(field_name.as_str(), field_value);
                }
            }
        }

        collection.last_index = collection.len() as i32;
        Ok(collection)
    }
}
