use std::collections::HashMap;
use std::io::Seek;

use rusqlite::{Connection, DatabaseName};
use rusqlite::blob::Blob;

use crate::audacity::fields::{FieldType, ReadDictField};
use crate::audacity::decoder::Decoder;


#[derive(Debug)]
pub struct TagDictReader {}


impl TagDictReader {
    pub fn new() -> Self {
        Self { }
    }
}


impl ReadDictField for TagDictReader {
    fn read_field(&self, blob: &mut Blob) -> FieldType {
        match blob.field_type_code() {
             0 => self.char_size(blob),
            15 => self.name(blob),
             _ => panic!("Unknown field type code detected"),
        }
    }

    fn char_size(&self, blob: &mut Blob) -> FieldType {
        FieldType::CharSize { value: blob.byte() }
    }

    fn name(&self, blob: &mut Blob) -> FieldType {
        let id = blob.short();
        let size = blob.short();
        FieldType::Name { id: id, size: size, value: blob.string(size as usize) }
    }
}

#[derive(Debug)]
pub struct TagDict {
    pub char_size: u8,
    pub dict: HashMap<i16, String>,
    read: TagDictReader,
}


impl TagDict {
    pub fn new() -> Self {
        Self {
            char_size: 0,
            dict: HashMap::new(),
            read: TagDictReader::new(),
        }
    }

    pub fn decode(&mut self, con: &Connection) {

        let mut blob = con.blob_open(DatabaseName::Main, "project",
            "dict", 1, true).expect("Failed to read blob");

        while (blob.stream_position().expect("Cannot read") as usize) < blob.len() {
            match self.read.read_field(&mut blob) {
                FieldType::CharSize { value } => { self.char_size = value; },
                FieldType::Name { id, value, .. } => { self.dict.insert(id, value.clone()); },
                _ => { panic!("asdfasdfasdf"); }
            }
        }
    }
}
