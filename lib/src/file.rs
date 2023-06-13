extern crate fs2;

use std::{fs, io};
use std::io::{Read, Write};

use bincode::config;
use fs2::FileExt;

use crate::collection::Collection;

pub struct File {}

impl File {
    pub fn save(collection: &Collection, file_name: &str) -> Result<(), io::Error> {
        let mut file = fs::File::create(file_name)?;
        let bytes = bincode::encode_to_vec(collection, config::standard()).expect("Valide collection");

        file.lock_exclusive()?;
        file.write_all(&bytes)?;
        file.unlock()?;

        Ok(())
    }

    pub fn load(file_name: &str) -> Result<Collection, io::Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut file = fs::File::open(file_name)?;

        file.lock_exclusive()?;
        file.read_to_end(&mut bytes)?;
        file.unlock()?;

        let collection: Collection = bincode::decode_from_slice(&bytes, config::standard()).expect("Valide db file").0;
        Ok(collection)
    }
}
